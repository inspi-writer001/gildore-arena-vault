use wincode::{SchemaWrite, SchemaRead};
use solana_address::Address;

#[derive(SchemaWrite, SchemaRead)]
pub struct UserState {
    pub user_address: Address,
    pub agent_id: Address,
    pub ticker_id: Address,
    pub is_initialized: PodBool,
    pub modified_time: u64,
    pub created_time: u64,
    pub amount: u64,
    pub bump: u8,
}
