use wincode::{SchemaWrite, SchemaRead};

#[derive(SchemaWrite, SchemaRead)]
pub struct Ticker {
    pub amount_to_spend: u64,
}
