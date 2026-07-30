use quasar_lang::{
    account_layout::AccountLayout, cpi::Seed, ops::close::close_account, prelude::*,
    sysvars::Sysvar,
};

use crate::{
    hashing::{sol_sha256, SolBytes},
    state::{GlobalState, IAgent},
};

#[derive(Accounts)]
pub struct Agent {
    #[account(mut)]
    pub admin: Signer,

    #[account(mut)]
    pub agent: UncheckedAccount,

    #[account(
        address = GlobalState::seeds(),
    )]
    pub global_state_account: Account<GlobalState>,

    pub system_program: Program<SystemProgram>,
}

impl Agent {
    #[inline(always)]
    pub fn register_agent(&mut self, name: &str) -> Result<(), ProgramError> {
        // ensure caller is an admin
        if !self
            .global_state_account
            .admin()
            .iter()
            .any(|a| a.eq(self.admin.address()))
        {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Pass multiple slices to the syscall without allocating a new Vec
        let slices = [
            SolBytes {
                ptr: crate::ID.as_ref().as_ptr(),
                len: 32,
            },
            SolBytes {
                ptr: name.as_ptr(),
                len: name.len() as u64,
            },
        ];

        let mut hash_result = [0u8; 32];

        unsafe {
            sol_sha256(
                slices.as_ptr() as *const u8,
                slices.len() as u64,
                hash_result.as_mut_ptr(),
            );
        }

        // let zeroed_out: [u8; 32] = [
        //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        //     0, 0, 0,
        // ];

        if Address::new_from_array(hash_result).eq(&Address::default()) {
            return Err(ProgramError::InvalidArgument);
        }

        let agent_id = Address::new_from_array(hash_result);

        let (agent_address, bump) = solana_address::Address::derive_program_address(
            &[b"agent", agent_id.as_ref()],
            &crate::ID,
        )
        .ok_or(ProgramError::InvalidSeeds)?;

        if agent_address != *self.agent.address() {
            return Err(ProgramError::InvalidSeeds);
        }

        let mut saved_seeds: [u8; 37] = [0u8; 37];

        saved_seeds[0..5].copy_from_slice(b"agent");
        saved_seeds[5..37].copy_from_slice(agent_id.as_ref());
        // [b"agent", agent_id.as_ref()]
        //     .concat()
        //     .try_into()
        //     .map_err(|_| ProgramError::InvalidArgument)?;

        // how to load agent_address as an account then write to it's data

        #[repr(C)]
        struct SeedStruct {
            pub first_seed: [u8; 5],
            pub second_seed: [u8; 32],
        }

        let mut first_seed = [0u8; 5];
        first_seed.copy_from_slice(&saved_seeds[0..5]);

        let mut second_seed = [0u8; 32];
        second_seed.copy_from_slice(&saved_seeds[5..]);

        let new_struct = SeedStruct {
            first_seed,
            second_seed,
        };

        if &new_struct.first_seed != b"agent" {
            return Err(ProgramError::InvalidArgument);
        }

        if &new_struct.second_seed != agent_id.as_ref() {
            return Err(ProgramError::InvalidArgument);
        }

        let rent = Rent::get()?;
        let minimum_balance: u64 = rent.try_minimum_balance(IAgent::SPACE).unwrap();

        let new_bump = &[bump];

        let account_seeds = [
            Seed::from(&new_struct.first_seed),
            Seed::from(&new_struct.second_seed),
            Seed::from(new_bump),
        ];

        self.system_program
            .create_account(
                &self.admin,
                &self.agent,
                minimum_balance,
                IAgent::SPACE as u64,
                &crate::ID,
            )
            .invoke_signed(&account_seeds)?;

        // let offset = IAgent::DATA_OFFSET;
        // let data_struct: IAgentInner = IAgentInner {
        //     agent_id,
        //     bump,
        //     seeds: saved_seeds,
        // };

        self.agent.write_bytes(0, &IAgent::DISCRIMINATOR)?; // account discriminator

        // self.agent.write_bytes(0, &IAgent::DISCRIMINATOR)?;

        // agent_id starts at DATA_OFFSET = 1
        self.agent
            .write_bytes(IAgent::DATA_OFFSET, agent_id.as_ref())?;

        // bump is at BUMP_OFFSET = 33
        self.agent
            .write_bytes(IAgent::BUMP_OFFSET.unwrap(), &[bump])?;

        // seeds come immediately after bump
        let seeds_offset = IAgent::BUMP_OFFSET.unwrap() + 1;
        self.agent.write_bytes(seeds_offset, &saved_seeds)?;

        // let data_bytes: &[u8] = unsafe {
        //     slice::from_raw_parts(
        //         (&data_struct as *const IAgentInner) as *const u8,
        //         size_of::<IAgentInner>(),
        //     )
        // };

        // assert_eq!(data_bytes.len(), IAgent::DATA_SIZE);

        // self.agent.write_bytes(offset, data_bytes)?;

        Ok(())
    }

    #[inline(always)]
    pub fn delete_agent(&mut self, id: Address) -> Result<(), ProgramError> {
        // assert caller is an admin
        if !self
            .global_state_account
            .admin()
            .iter()
            .any(|a| a.eq(self.admin.address()))
        {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // assert account is valid
        {
            let (expected_agent_address, _) =
                Address::derive_program_address(&[b"agent", id.as_ref()], &crate::ID)
                    .ok_or(ProgramError::InvalidSeeds)?;

            if expected_agent_address != *self.agent.address() {
                return Err(ProgramError::InvalidSeeds);
            }

            let agent_struct = unsafe { IAgent::from_view_unchecked(self.agent.to_account_view()) };

            assert!(agent_struct.agent_id == id, "failed account comparison");
        }

        // core::ptr::slice_from_raw_parts::<IAgent>(agent, IAgent::DATA_SIZE);

        close_account(
            unsafe { self.agent.to_account_view_mut() },
            self.admin.to_account_view(),
            IAgent::DISCRIMINATOR.len(),
        )?;

        Ok(())
    }
}
