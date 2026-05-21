use wincode::{SchemaWrite, SchemaRead};
use solana_address::Address;
use quasar_lang::client::{DynVec};

#[derive(SchemaWrite, SchemaRead)]
pub struct InitializeArgs {
    pub fee_bps: u16,
    pub max_fee: u64,
    pub admin: DynVec<Address, u16>,
}
