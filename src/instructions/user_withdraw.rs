use quasar_lang::{cpi::Seed, prelude::*, sysvars::Sysvar};
use quasar_spl::prelude::*;

use crate::state::{GlobalState, IAgent, UserState};

#[derive(Accounts)]
pub struct UserWithdrawFromParticularAgentVault {
    #[account(mut)]
    pub user: Signer,

    #[account(
        address= IAgent::seeds(agent.agent_id())
    )]
    pub agent: Account<IAgent>,

    #[account(
       mut,
       address = UserState::seeds(user.address(),  mint.address(), agent.address()),
    )]
    pub user_state: Account<UserState>,

    #[account(
       constraints(&user_state_vault.mint == mint.address()),
       constraints(&user_state_vault.owner == user_state.address())
    )]
    pub user_state_vault: InterfaceAccount<Token>,

    pub mint: InterfaceAccount<Mint>,

    #[account(
        mut,
        address = GlobalState::seeds(),
    )]
    pub global_state_account: Account<GlobalState>,

    #[account(mut)]
    pub user_token_account: InterfaceAccount<Token>,
    pub token_program: Interface<TokenInterface>,
}

impl UserWithdrawFromParticularAgentVault {
    pub fn withdraw(&mut self, amount: u64) -> Result<(), ProgramError> {
        let user_vault_amount = self.user_state_vault.amount();
        assert!(user_vault_amount >= amount, "cannot withdraw below balance");
        let updated_user_state_amount = self
            .user_state
            .net_deposited_amount()
            .checked_sub(amount)
            .ok_or(ProgramError::InvalidArgument)?;

        let user_address = self.user.address();
        let mint = self.mint.address();
        let agent_address = self.agent.address();

        let (expected_user_state, bump) = Address::derive_program_address(
            &[
                b"user_state",
                user_address.as_ref(),
                mint.as_ref(),
                agent_address.as_ref(),
            ],
            &crate::ID,
        )
        .ok_or(ProgramError::InvalidSeeds)?;

        if expected_user_state != *self.user_state.address() {
            return Err(ProgramError::InvalidSeeds);
        }

        let bump = [bump];

        let seeds = [
            Seed::from(b"user_state" as &[u8]),
            Seed::from(user_address.as_ref()),
            Seed::from(mint.as_ref()),
            Seed::from(agent_address.as_ref()),
            Seed::from(bump.as_ref()),
        ];

        self.token_program
            .transfer_checked(
                &self.user_state_vault,
                &self.mint,
                &self.user_token_account,
                &self.user_state,
                amount,
                self.mint.decimals,
            )
            .invoke_signed(&seeds)?;

        let current_time = i64::from(Clock::get()?.unix_timestamp) as u64;

        self.user_state.net_deposited_amount = updated_user_state_amount.into();
        self.user_state.modified_time = current_time.into();

        Ok(())
    }
}
