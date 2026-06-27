use std::vec;
use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};

pub const ID: Address = solana_address::address!("2um3F4vyQwcuhwrGdPHGMwwK5C4K5rFU84cxHPoYNMKg");

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
            AccountMeta::new_readonly(ix.global_state_account, false),
            AccountMeta::new(ix.destination_token_account, false),
            AccountMeta::new_readonly(ix.system_program, false),
        ];
        let mut data = vec![0];
        data.extend_from_slice(&ix.args.to_le_bytes());
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}

pub struct RegisterAgentInstruction {
    pub admin: Address,
    pub agent: Address,
    pub global_state_account: Address,
    pub system_program: Address,
    pub name: PodString,
}

impl From<RegisterAgentInstruction> for Instruction {
    fn from(ix: RegisterAgentInstruction) -> Instruction {
        let accounts = vec![
            AccountMeta::new(ix.admin, true),
            AccountMeta::new(ix.agent, false),
            AccountMeta::new_readonly(ix.global_state_account, false),
            AccountMeta::new_readonly(ix.system_program, false),
        ];
        let mut data = vec![1];
        data.extend_from_slice(&ix.name.to_le_bytes());
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}

pub struct DeleteAgentInstruction {
    pub admin: Address,
    pub agent: Address,
    pub global_state_account: Address,
    pub system_program: Address,
    pub id: Address,
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
        data.extend_from_slice(ix.id.as_ref());
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}

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
        data.extend_from_slice(&ix.amount.to_le_bytes());
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}

pub struct RegisterTickerForMeInstruction {
    pub payer: Address,
    pub user: Address,
    pub agent: Address,
    pub user_state: Address,
    pub ticker: Address,
    pub mint: Address,
    pub amount_to_spend: u64,
}

impl From<RegisterTickerForMeInstruction> for Instruction {
    fn from(ix: RegisterTickerForMeInstruction) -> Instruction {
        let accounts = vec![
            AccountMeta::new(ix.payer, true),
            AccountMeta::new(ix.user, true),
            AccountMeta::new(ix.agent, false),
            AccountMeta::new_readonly(ix.user_state, false),
            AccountMeta::new_readonly(ix.ticker, false),
            AccountMeta::new_readonly(ix.mint, false),
        ];
        let mut data = vec![4];
        data.extend_from_slice(&ix.amount_to_spend.to_le_bytes());
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}

pub struct ConsumeTickerInstruction {
    pub broadcaster: Address,
    pub admin: Address,
    pub user: Address,
    pub agent: Address,
    pub global_state_account: Address,
    pub user_state: Address,
    pub user_state_vault: Address,
    pub destination: Address,
    pub ticker: Address,
    pub mint: Address,
    pub token_program: Address,
    pub system_program: Address,
}

impl From<ConsumeTickerInstruction> for Instruction {
    fn from(ix: ConsumeTickerInstruction) -> Instruction {
        let accounts = vec![
            AccountMeta::new(ix.broadcaster, true),
            AccountMeta::new(ix.admin, true),
            AccountMeta::new_readonly(ix.user, false),
            AccountMeta::new(ix.agent, false),
            AccountMeta::new(ix.global_state_account, false),
            AccountMeta::new(ix.user_state, false),
            AccountMeta::new(ix.user_state_vault, false),
            AccountMeta::new(ix.destination, false),
            AccountMeta::new(ix.ticker, false),
            AccountMeta::new_readonly(ix.mint, false),
            AccountMeta::new_readonly(ix.token_program, false),
            AccountMeta::new_readonly(ix.system_program, false),
        ];
        let data = vec![5];
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}

pub struct UserWithdrawalInstruction {
    pub user: Address,
    pub agent: Address,
    pub user_state: Address,
    pub user_state_vault: Address,
    pub mint: Address,
    pub global_state_account: Address,
    pub user_token_account: Address,
    pub token_program: Address,
    pub amount: u64,
}

impl From<UserWithdrawalInstruction> for Instruction {
    fn from(ix: UserWithdrawalInstruction) -> Instruction {
        let accounts = vec![
            AccountMeta::new(ix.user, true),
            AccountMeta::new_readonly(ix.agent, false),
            AccountMeta::new(ix.user_state, false),
            AccountMeta::new_readonly(ix.user_state_vault, false),
            AccountMeta::new_readonly(ix.mint, false),
            AccountMeta::new(ix.global_state_account, false),
            AccountMeta::new(ix.user_token_account, false),
            AccountMeta::new_readonly(ix.token_program, false),
        ];
        let mut data = vec![6];
        data.extend_from_slice(&ix.amount.to_le_bytes());
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}

pub struct UpdateTickerCloseTradeInstruction {
    pub broadcaster: Address,
    pub user: Address,
    pub agent: Address,
    pub ticker: Address,
}

impl From<UpdateTickerCloseTradeInstruction> for Instruction {
    fn from(ix: UpdateTickerCloseTradeInstruction) -> Instruction {
        let accounts = vec![
            AccountMeta::new(ix.broadcaster, true),
            AccountMeta::new_readonly(ix.user, false),
            AccountMeta::new(ix.agent, false),
            AccountMeta::new(ix.ticker, false),
        ];
        let data = vec![7];
        Instruction {
            program_id: ID,
            accounts,
            data,
        }
    }
}

