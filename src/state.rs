use quasar_lang::prelude::*;

pub const GLOBAL_STATE_SEED: &[u8; 12] = b"global_state";
pub const DEPLOYER_ADDRESS: Address =
    Address::from_str_const("mStr9x2XYUNQwwTmNk67Rm4ExgGRjhkrNT86SjhhUvt");

#[account(discriminator = 1, set_inner)]
#[seeds(b"user_state", user: Address,token_mint: Address, agent_id: Address)]
pub struct UserState {
    pub user_address: Address,
    pub agent_id: Address, // if agent is locked in, no withdrawals from vault
    pub ticker_id: Address,
    pub is_initialized: PodBool,
    pub modified_time: u64,
    pub created_time: u64,
    pub amount: u64,
    pub bump: u8,
}

#[account(discriminator = 2, set_inner)]
#[seeds(b"global_state")]
pub struct GlobalState {
    /// Token Account
    pub fee_destination: Address,
    pub fee_bps: u16,
    pub max_fee: u64,
    pub bump: u8,
    pub admin: Vec<Address, 4>,
}

#[account(discriminator = 3, set_inner)]
#[seeds(b"market", agent_id: Address, ticker: Address)]
pub struct AgentMarket {
    pub agent_id: Address,
    pub ticker_id: Address,
    pub is_trading: bool,
    pub bump: u8,
}

#[account(discriminator = 4, set_inner)]
#[seeds(b"agent", agent_id: Address)]
pub struct IAgent {
    pub agent_id: Address,
    pub bump: u8,
    pub seeds: [u8; 37],
}

#[account(discriminator = 5)]
#[seeds(b"ticker", agent_id: Address, user: Address)]
pub struct Ticker {
    pub amount_to_spend: u64,
}
