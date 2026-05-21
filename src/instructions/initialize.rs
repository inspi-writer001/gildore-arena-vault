use crate::state::{GlobalState, GlobalStateInner, DEPLOYER_ADDRESS};
use quasar_lang::{prelude::*, sysvars::Sysvar};
use quasar_spl::Token;

#[derive(Accounts)]
pub struct Initialize {
    #[account(mut, address = DEPLOYER_ADDRESS)]
    pub payer: Signer,

    #[account(
        init,
       address = GlobalState::seeds(),
    )]
    pub global_state_account: Account<GlobalState>,

    #[account(mut)]
    pub destination_token_account: InterfaceAccount<Token>,
    pub system_program: Program<SystemProgram>,
}

impl Initialize {
    #[inline(always)]
    pub fn initialize(
        &mut self,
        args: InitializeArgs,
        bumps: InitializeBumps,
    ) -> Result<(), ProgramError> {
        assert!(args.fee_bps <= 1_500, "fee exceeds 15%");

        let rent = Rent::get()?;

        self.global_state_account.set_inner(
            GlobalStateInner {
                admin: &args.admin,
                bump: bumps.global_state_account,
                fee_bps: args.fee_bps,
                max_fee: args.max_fee,
                fee_destination: *self.destination_token_account.address(),
            },
            self.payer.to_account_view(),
            rent.lamports_per_byte(),
            rent.exemption_threshold_raw(),
        )?;
        Ok(())
    }
}

// #[repr(C)]
#[derive(QuasarSerialize)]
pub struct InitializeArgs {
    pub fee_bps: u16,
    pub max_fee: u64,
    pub admin: Vec<Address, 4>,
}

impl InitializeArgs {
    #[cfg(not(any(target_os = "solana", target_arch = "bpf")))]
    pub fn to_bytes(&self) -> [u8; InitializeArgs::POD_SIZE] {
        unsafe {
            let data = core::slice::from_raw_parts(
                (self as *const InitializeArgs) as *const u8,
                InitializeArgs::POD_SIZE,
            );
            let mut data_ = [0u8; InitializeArgs::POD_SIZE];
            data_.copy_from_slice(&data);

            data_
        }
    }
}
