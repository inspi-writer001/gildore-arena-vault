use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};
use crate::ID;

pub struct RegisterTickerForMeInstruction {
    pub payer: Address,
    pub user: Address,
    pub agent: Address,
    pub user_state: Address,
    pub user_state_vault: Address,
    pub ticker: Address,
    pub mint: Address,
    pub token_program: Address,
    pub system_program: Address,
    pub amount_to_spend: u64,
}

impl From<RegisterTickerForMeInstruction> for Instruction {
    fn from(ix: RegisterTickerForMeInstruction) -> Instruction {
        let accounts = vec![
            AccountMeta::new(ix.payer, true),
            AccountMeta::new(ix.user, true),
            AccountMeta::new(ix.agent, false),
            AccountMeta::new_readonly(ix.user_state, false),
            AccountMeta::new_readonly(ix.user_state_vault, false),
            AccountMeta::new_readonly(ix.ticker, false),
            AccountMeta::new_readonly(ix.mint, false),
            AccountMeta::new_readonly(ix.token_program, false),
            AccountMeta::new_readonly(ix.system_program, false),
        ];
        let mut data = vec![4];
        wincode::serialize_into(&mut data, &ix.amount_to_spend).expect("serialization into Vec<u8> is infallible");
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}
