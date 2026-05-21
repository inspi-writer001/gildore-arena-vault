use wincode::{SchemaWrite, SchemaRead};
use solana_address::Address;

#[derive(SchemaWrite, SchemaRead)]
pub struct AgentMarket {
    pub agent_id: Address,
    pub ticker_id: Address,
    pub is_trading: bool,
    pub bump: u8,
}
