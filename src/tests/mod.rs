use quasar_svm::{token::create_keyed_mint_account_with_program, Pubkey, QuasarSvm};
use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};
use spl_token_interface::{
    state::Account as TokenAccount,
    state::{AccountState, Mint},
    ID as TOKEN_PROGRAM_ID,
};
use zeropod::Vec;

use crate::{
    instructions::InitializeArgs,
    state::{DEPLOYER_ADDRESS, GLOBAL_STATE_SEED},
};

mod initialize;
mod register_agent;
mod deposit_test;
mod create_ticker;

pub struct ReusableState {
    pub global_state: Address,
    pub payer: Address,
    pub token_a_mint: Address,
    pub token_a_address: Address,
    pub fee_token_account: Address,
}

pub fn setup() -> (QuasarSvm, ReusableState) {
    let elf = std::fs::read("target/deploy/gildore_vault.so").unwrap();

    let payer = DEPLOYER_ADDRESS;
    let mint = Pubkey::new_unique();

    let (global_state, _) = Pubkey::find_program_address(&[GLOBAL_STATE_SEED.as_ref()], &crate::ID);

    let token_mint = create_keyed_mint_account_with_program(
        &mint,
        &Mint {
            supply: 10_000_000,
            decimals: 9,
            freeze_authority: None.into(),
            is_initialized: true,
            mint_authority: Some(payer).into(),
        },
        &TOKEN_PROGRAM_ID,
    );

    let (user_token_account, _) = Pubkey::find_program_address(
        &[payer.as_ref(), TOKEN_PROGRAM_ID.as_ref(), mint.as_ref()],
        &TOKEN_PROGRAM_ID,
    );

    let reusable_state = ReusableState {
        global_state,
        payer,
        token_a_mint: token_mint.address,
        token_a_address: user_token_account,
        fee_token_account: user_token_account,
    };

    let mut injected_svm = QuasarSvm::new().with_program(&Pubkey::from(crate::ID), &elf);

    injected_svm.airdrop(&payer, 100_000000000);
    let returned_setup = (injected_svm, reusable_state);

    returned_setup
}

// fn initialize_instruction(
//     payer: Address,
//     global_state: Address,
//     destination_token_address: Address,
//     system_program: Address,
// ) -> Instruction {
//     let mut admins: Vec<Address, 4> = Vec::<Address, 4>::default();
//     let _ = admins.push(DEPLOYER_ADDRESS);

//     let init_arguments = InitializeArgs {
//         fee_bps: 10, // 0.1% fee
//         max_fee: 20, // 20usd
//         admin: admins,
//     };

//     let mut instruction_data = vec![1u8];
//     instruction_data.extend_from_slice(&init_arguments.to_bytes());

//     Instruction {
//         program_id: Address::from(crate::ID.to_bytes()),
//         accounts: vec![
//             AccountMeta::new(payer, true),
//             AccountMeta::new(global_state, false),
//             AccountMeta::new(destination_token_address, false),
//             AccountMeta::new_readonly(system_program, false),
//         ],
//         data: instruction_data,
//     }
// }
