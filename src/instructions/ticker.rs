use quasar_lang::prelude::*;
use quasar_spl::prelude::*;

use crate::state::{GlobalState, IAgent, Ticker, UserState};

#[derive(Accounts)]
pub struct ATicker {
    #[account(mut)]
    pub payer: Signer,

    #[account(mut)]
    pub user: Signer,

    #[account(
        mut,
        address= IAgent::seeds(agent.agent_id())
    )]
    pub agent: Account<IAgent>,

    #[account(
       address = UserState::seeds(user.address(),  mint.address(), agent.address()),
    )]
    pub user_state: Account<UserState>,

    #[account(
        token(
            mint = mint, authority = user_state, token_program = token_program
        )
    )]
    pub user_state_vault: InterfaceAccount<Token>,

    #[account(
        init(idempotent),
        payer = payer,
        address = Ticker::seeds(agent.agent_id(), user.address())
    )]
    pub ticker: Account<Ticker>,

    pub mint: InterfaceAccount<Mint>,

    pub token_program: Interface<TokenInterface>,
    pub system_program: Program<SystemProgram>,
}

impl ATicker {
    pub fn register_ticker_for_me(&mut self, amount_to_spend: u64) -> Result<(), ProgramError> {
        if self.user_state_vault.amount() > 0 {
            assert!(
                amount_to_spend <= self.user_state_vault.amount(),
                "cannot approve more than balance"
            );
        } // we will relax this check to only when there's a balance because we need this to be called before deposit where the user_state_vault will be created

        self.ticker.amount_to_spend = amount_to_spend.into();
        Ok(())
    }
}
