use quasar_lang::prelude::*;
use quasar_spl::prelude::*;

use crate::state::{IAgent, Ticker, UserState};

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
        init(idempotent),
        payer = payer,
        address = Ticker::seeds(agent.agent_id(), user.address())
    )]
    pub ticker: Account<Ticker>,

    pub mint: InterfaceAccount<Mint>,
}

impl ATicker {
    #[inline(always)]
    pub fn register_ticker_for_me(&mut self, amount_to_spend: u64) -> Result<(), ProgramError> {
        self.ticker.amount_to_spend = amount_to_spend.into();
        Ok(())
    }
}
