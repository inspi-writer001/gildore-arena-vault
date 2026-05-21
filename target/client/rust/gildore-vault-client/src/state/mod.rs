pub mod user_state;
pub mod global_state;
pub mod agent_market;
pub mod i_agent;
pub mod ticker;

pub use user_state::*;
pub use global_state::*;
pub use agent_market::*;
pub use i_agent::*;
pub use ticker::*;

pub enum ProgramAccount {
    UserState(UserState),
    GlobalState(GlobalState),
    AgentMarket(AgentMarket),
    IAgent(IAgent),
    Ticker(Ticker),
}

pub fn decode_account(data: &[u8]) -> Option<ProgramAccount> {
    if data.starts_with(USER_STATE_ACCOUNT_DISCRIMINATOR) {
        return wincode::deserialize::<UserState>(data).ok().map(ProgramAccount::UserState);
    }
    if data.starts_with(GLOBAL_STATE_ACCOUNT_DISCRIMINATOR) {
        return wincode::deserialize::<GlobalState>(data).ok().map(ProgramAccount::GlobalState);
    }
    if data.starts_with(AGENT_MARKET_ACCOUNT_DISCRIMINATOR) {
        return wincode::deserialize::<AgentMarket>(data).ok().map(ProgramAccount::AgentMarket);
    }
    if data.starts_with(I_AGENT_ACCOUNT_DISCRIMINATOR) {
        return wincode::deserialize::<IAgent>(data).ok().map(ProgramAccount::IAgent);
    }
    if data.starts_with(TICKER_ACCOUNT_DISCRIMINATOR) {
        return wincode::deserialize::<Ticker>(data).ok().map(ProgramAccount::Ticker);
    }
    None
}
