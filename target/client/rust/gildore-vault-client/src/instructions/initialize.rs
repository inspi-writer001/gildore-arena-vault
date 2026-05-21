use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};
use crate::ID;
use crate::types::InitializeArgs;

pub struct InitializeInstruction {
    pub payer: Address,
    pub global_state_account: Address,
    pub destination_token_account: Address,
    pub system_program: Address,
    pub args: InitializeArgs,
}

impl From<InitializeInstruction> for Instruction {
    fn from(ix: InitializeInstruction) -> Instruction {
        let accounts = vec![
            AccountMeta::new(ix.payer, true),
            AccountMeta::new(ix.global_state_account, false),
            AccountMeta::new(ix.destination_token_account, false),
            AccountMeta::new_readonly(ix.system_program, false),
        ];
        let mut data = vec![0];
        wincode::serialize_into(&mut data, &ix.args).expect("serialization into Vec<u8> is infallible");
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}
