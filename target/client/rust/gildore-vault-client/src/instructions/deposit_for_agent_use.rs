use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};
use crate::ID;

pub struct DepositForAgentUseInstruction {
    pub payer: Address,
    pub user: Address,
    pub agent: Address,
    pub global_state_account: Address,
    pub user_state: Address,
    pub user_state_vault: Address,
    pub ticker: Address,
    pub destination_fee_token_account: Address,
    pub user_token_account: Address,
    pub mint: Address,
    pub token_program: Address,
    pub system_program: Address,
    pub amount: u64,
}

impl From<DepositForAgentUseInstruction> for Instruction {
    fn from(ix: DepositForAgentUseInstruction) -> Instruction {
        let accounts = vec![
            AccountMeta::new(ix.payer, true),
            AccountMeta::new(ix.user, true),
            AccountMeta::new(ix.agent, false),
            AccountMeta::new(ix.global_state_account, false),
            AccountMeta::new_readonly(ix.user_state, false),
            AccountMeta::new_readonly(ix.user_state_vault, false),
            AccountMeta::new_readonly(ix.ticker, false),
            AccountMeta::new(ix.destination_fee_token_account, false),
            AccountMeta::new(ix.user_token_account, false),
            AccountMeta::new_readonly(ix.mint, false),
            AccountMeta::new_readonly(ix.token_program, false),
            AccountMeta::new_readonly(ix.system_program, false),
        ];
        let mut data = vec![3];
        wincode::serialize_into(&mut data, &ix.amount).expect("serialization into Vec<u8> is infallible");
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}
