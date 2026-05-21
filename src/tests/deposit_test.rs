use core::fmt::Error;

use crate::{
    cpi::DepositForAgentUseInstruction,
    hashing::{sol_sha256, SolBytes},
    tests::{
        initialize::process_initialize, register_agent::process_register_agent, setup,
        ReusableState,
    },
};
use quasar_lang::prelude::*;
use quasar_svm::{
    system_program,
    token::{create_keyed_mint_account_with_program, create_keyed_token_account},
    Account, Instruction, Pubkey,
};
use spl_token_interface::{
    state::{Account as TokenAccount, AccountState, Mint},
    ID as TOKEN_PROGRAM_ID,
};

const TEST_AGENT_NAME: &str = "alpha-bot";
const DEPOSIT_AMOUNT: u64 = 100_000;
const INITIAL_USER_BALANCE: u64 = 200_000;
const EXPECTED_PLATFORM_FEE: u64 = 20;
const EXPECTED_VAULT_AMOUNT: u64 = DEPOSIT_AMOUNT - EXPECTED_PLATFORM_FEE;

fn derive_agent_id(name: &str) -> Address {
    let slices = [
        SolBytes {
            ptr: crate::ID.as_ref().as_ptr(),
            len: 32,
        },
        SolBytes {
            ptr: name.as_ptr(),
            len: name.len() as u64,
        },
    ];

    let mut hash_result = [0u8; 32];

    unsafe {
        sol_sha256(
            slices.as_ptr() as *const u8,
            slices.len() as u64,
            hash_result.as_mut_ptr(),
        );
    }

    Address::new_from_array(hash_result)
}

pub struct DepositContext {
    pub user: Address,
    pub agent: Address,
    pub user_state: Address,
    pub user_state_vault: Address,
    pub ticker: Address,
    pub user_token_account: Address,
    pub fee_token_account: Address,
    pub mint: Address,
    pub deposited_amount: u64,
    pub expected_fee: u64,
    pub expected_vault_amount: u64,
}

pub fn process_deposit_for_agent_use(
    svm: &mut quasar_svm::QuasarSvm,
    payer: Address,
    global_state: Address,
    fee_token_account: Address,
    mint: Address,
) -> Result<DepositContext, Error> {
    let user = Pubkey::new_unique();
    let user_token_account = Pubkey::new_unique();
    let user_state_vault = Pubkey::new_unique();

    svm.airdrop(&user, 10_000_000_000);

    let agent_id = derive_agent_id(TEST_AGENT_NAME);
    let (agent, _) = Address::derive_program_address(&[b"agent", agent_id.as_ref()], &crate::ID)
        .expect("agent PDA should derive");
    let (user_state, _) = Address::derive_program_address(
        &[b"user_state", user.as_ref(), mint.as_ref(), agent.as_ref()],
        &crate::ID,
    )
    .expect("user_state PDA should derive");
    let (ticker, _) =
        Address::derive_program_address(&[b"ticker", agent_id.as_ref(), user.as_ref()], &crate::ID)
            .expect("ticker PDA should derive");

    let deposit_instruction: Instruction = DepositForAgentUseInstruction {
        payer,
        user,
        agent,
        global_state_account: global_state,
        user_state,
        user_state_vault,
        ticker,
        destination_fee_token_account: fee_token_account,
        user_token_account,
        mint,
        token_program: quasar_svm::SPL_TOKEN_PROGRAM_ID,
        system_program: system_program::ID,
        amount: DEPOSIT_AMOUNT,
    }
    .into();

    let result = svm.process_instruction(
        &deposit_instruction,
        &[
            svm.get_account(&payer).expect("payer account"),
            svm.get_account(&user).expect("user account"),
            svm.get_account(&agent).expect("agent account"),
            svm.get_account(&global_state).expect("global state account"),
            Account {
                address: user_state,
                lamports: 0,
                data: vec![],
                executable: false,
                owner: system_program::ID,
            },
            Account {
                address: user_state_vault,
                lamports: 0,
                data: vec![],
                executable: false,
                owner: system_program::ID,
            },
            Account {
                address: ticker,
                lamports: 0,
                data: vec![],
                executable: false,
                owner: system_program::ID,
            },
            svm.get_account(&fee_token_account)
                .expect("fee token account"),
            create_keyed_token_account(
                &user_token_account,
                &TokenAccount {
                    mint,
                    owner: user,
                    amount: INITIAL_USER_BALANCE,
                    state: AccountState::Initialized,
                    ..TokenAccount::default()
                },
            ),
            create_keyed_mint_account_with_program(
                &mint,
                &Mint {
                    supply: 10_000_000,
                    decimals: 9,
                    freeze_authority: None.into(),
                    is_initialized: true,
                    mint_authority: Some(payer).into(),
                },
                &TOKEN_PROGRAM_ID,
            ),
        ],
    );

    print!("deposit_for_agent_use CUs: {:?} \n", result.compute_units_consumed);
    print!("deposit_for_agent_use logs: {:?} \n", result.logs);
    result.assert_success();

    Ok(DepositContext {
        user,
        agent,
        user_state,
        user_state_vault,
        ticker,
        user_token_account,
        fee_token_account,
        mint,
        deposited_amount: DEPOSIT_AMOUNT,
        expected_fee: EXPECTED_PLATFORM_FEE,
        expected_vault_amount: EXPECTED_VAULT_AMOUNT,
    })
}

#[test]
fn deposit_for_agent_use_test() {
    let (mut svm, reusable_state) = setup();

    let ReusableState {
        fee_token_account,
        global_state,
        payer,
        token_a_mint,
        ..
    } = reusable_state;

    process_initialize(
        &mut svm,
        fee_token_account,
        global_state,
        token_a_mint,
        payer,
    )
    .expect("error in initialize");

    process_register_agent(TEST_AGENT_NAME, &mut svm, global_state, payer)
        .expect("error in register_agent");

    let ctx = process_deposit_for_agent_use(
        &mut svm,
        payer,
        global_state,
        fee_token_account,
        token_a_mint,
    )
    .expect("error in deposit_for_agent_use");

    let user_state_account = svm.get_account(&ctx.user_state).expect("user state should exist");
    assert_eq!(user_state_account.owner, crate::ID);
    assert_eq!(user_state_account.data[0], 1);
    assert_eq!(
        Address::new_from_array(user_state_account.data[1..33].try_into().unwrap()),
        ctx.user
    );

    let expected_agent_id = derive_agent_id(TEST_AGENT_NAME);
    assert_eq!(
        Address::new_from_array(user_state_account.data[33..65].try_into().unwrap()),
        expected_agent_id
    );
    assert_eq!(
        Address::new_from_array(user_state_account.data[65..97].try_into().unwrap()),
        ctx.ticker
    );
    assert_eq!(user_state_account.data[97], 1);
    assert_eq!(
        u64::from_le_bytes(user_state_account.data[114..122].try_into().unwrap()),
        ctx.expected_vault_amount
    );

    let fee_token_account_data = svm.get_account(&ctx.fee_token_account).unwrap().data;
    let user_token_account_data = svm.get_account(&ctx.user_token_account).unwrap().data;
    let user_state_vault_data = svm.get_account(&ctx.user_state_vault).unwrap().data;

    assert_eq!(
        u64::from_le_bytes(fee_token_account_data[64..72].try_into().unwrap()),
        ctx.expected_fee
    );
    assert_eq!(
        u64::from_le_bytes(user_token_account_data[64..72].try_into().unwrap()),
        INITIAL_USER_BALANCE - ctx.deposited_amount
    );
    assert_eq!(
        u64::from_le_bytes(user_state_vault_data[64..72].try_into().unwrap()),
        ctx.expected_vault_amount
    );
}
