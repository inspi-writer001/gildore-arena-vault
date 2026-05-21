use wincode::{SchemaWrite, SchemaRead};
use solana_address::Address;

#[derive(SchemaWrite, SchemaRead)]
pub struct IAgent {
    pub agent_id: Address,
    pub bump: u8,
    pub seeds: [u8; 37],
}
