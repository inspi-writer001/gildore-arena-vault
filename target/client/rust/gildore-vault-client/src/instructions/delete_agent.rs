use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};
use crate::ID;
use quasar_lang::client::{DynString};

pub struct DeleteAgentInstruction {
    pub admin: Address,
    pub agent: Address,
    pub global_state_account: Address,
    pub system_program: Address,
    pub name: DynString<u8>,
}

impl From<DeleteAgentInstruction> for Instruction {
    fn from(ix: DeleteAgentInstruction) -> Instruction {
        let accounts = vec![
            AccountMeta::new(ix.admin, true),
            AccountMeta::new(ix.agent, false),
            AccountMeta::new_readonly(ix.global_state_account, false),
            AccountMeta::new_readonly(ix.system_program, false),
        ];
        let mut data = vec![2];
        data.extend_from_slice(&(ix.name.len() as u64).to_le_bytes()[..1]);
        data.extend_from_slice(ix.name.as_bytes());
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}
