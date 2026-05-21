use solana_address::Address;

pub const ID: Address = solana_address::address!("2um3F4vyQwcuhwrGdPHGMwwK5C4K5rFU84cxHPoYNMKg");

pub mod instructions;
pub mod state;
pub mod types;
pub mod errors;
pub mod pda;

pub use instructions::*;
pub use state::*;
pub use types::*;
pub use errors::*;
pub use pda::*;
