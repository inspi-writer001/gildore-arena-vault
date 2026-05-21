use wincode::{SchemaWrite, SchemaRead};
use solana_address::Address;
use quasar_lang::client::{DynVec};

#[derive(SchemaWrite, SchemaRead)]
pub struct GlobalState {
    pub fee_destination: Address,
    pub fee_bps: u16,
    pub max_fee: u64,
    pub bump: u8,
    pub admin: DynVec<Address, u16>,
}
