use quasar_lang::client::{DynString};
use crate::types::InitializeArgs;
pub mod initialize;
pub mod register_agent;
pub mod delete_agent;
pub mod deposit_for_agent_use;
pub mod register_ticker_for_me;

pub use initialize::*;
pub use register_agent::*;
pub use delete_agent::*;
pub use deposit_for_agent_use::*;
pub use register_ticker_for_me::*;

pub enum ProgramInstruction {
    Initialize { args: InitializeArgs },
    RegisterAgent { name: DynString<u8> },
    DeleteAgent { name: DynString<u8> },
    DepositForAgentUse { amount: u64 },
    RegisterTickerForMe { amount_to_spend: u64 },
}

pub fn decode_instruction(data: &[u8]) -> Option<ProgramInstruction> {
    let disc = *data.first()?;
    match disc {
        0 => {
            let payload = &data[1..];
            let args: InitializeArgs = wincode::deserialize(payload).ok()?;
            Some(ProgramInstruction::Initialize { args })
        }
        1 => {
            let payload = &data[1..];
            let mut offset = 0usize;
            let name_len = {
                let mut buf = [0u8; 8];
                buf[..1].copy_from_slice(&payload[offset..offset + 1]);
                offset += 1;
                u64::from_le_bytes(buf) as usize
            };
            let name: DynString<u8> = payload[offset..offset + name_len].to_vec().into();
            offset += name_len;
            Some(ProgramInstruction::RegisterAgent { name })
        }
        2 => {
            let payload = &data[1..];
            let mut offset = 0usize;
            let name_len = {
                let mut buf = [0u8; 8];
                buf[..1].copy_from_slice(&payload[offset..offset + 1]);
                offset += 1;
                u64::from_le_bytes(buf) as usize
            };
            let name: DynString<u8> = payload[offset..offset + name_len].to_vec().into();
            offset += name_len;
            Some(ProgramInstruction::DeleteAgent { name })
        }
        3 => {
            let payload = &data[1..];
            let amount: u64 = wincode::deserialize(payload).ok()?;
            Some(ProgramInstruction::DepositForAgentUse { amount })
        }
        4 => {
            let payload = &data[1..];
            let amount_to_spend: u64 = wincode::deserialize(payload).ok()?;
            Some(ProgramInstruction::RegisterTickerForMe { amount_to_spend })
        }
        _ => None,
    }
}
