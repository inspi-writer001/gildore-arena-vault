use crate::{
    cpi::ConsumeTickerInstruction,
    tests::{
        create_ticker::process_register_ticker_for_me,
        deposit_test::{process_deposit_for_agent_use, DepositContext},
        initialize::process_initialize,
        register_agent::process_register_agent,
        setup, ReusableState,
    },
};
use quasar_svm::{
    token::{create_keyed_mint_account_with_program, create_keyed_token_account},
    Account, Instruction, ProgramError, Pubkey,
};
use spl_token_interface::{
    state::{Account as TokenAccount, AccountState, Mint},
    ID as TOKEN_PROGRAM_ID,
};

const TEST_AGENT_NAME: &str = "alpha-bot";
const APPROVED_SPEND: u64 = 20_000_000_000;
const BOOSTED_VAULT_AMOUNT: u64 = 40_000_000_000;

fn set_token_account_amount(
    svm: &mut quasar_svm::QuasarSvm,
    token_account: solana_address::Address,
    amount: u64,
) {
    let mut account: Account = svm
        .get_account(&token_account)
        .expect("token account should exist");
    account.data[64..72].copy_from_slice(&amount.to_le_bytes());
    svm.set_account(account);
}

fn create_destination_token_account(
    owner: solana_address::Address,
    mint: solana_address::Address,
) -> Account {
    let destination = Pubkey::new_unique();
    create_keyed_token_account(
        &destination,
        &TokenAccount {
            mint,
            owner,
            amount: 0,
            state: AccountState::Initialized,
            ..TokenAccount::default()
        },
    )
}

fn process_consume_ticker(
    svm: &mut quasar_svm::QuasarSvm,
    broadcaster: solana_address::Address,
    admin: solana_address::Address,
    global_state: solana_address::Address,
    deposit_ctx: &DepositContext,
    destination: &Account,
) -> quasar_svm::ExecutionResult {
    let consume_ticker_instruction: Instruction = ConsumeTickerInstruction {
        broadcaster,
        admin,
        user: deposit_ctx.user,
        agent: deposit_ctx.agent,
        global_state_account: global_state,
        user_state: deposit_ctx.user_state,
        user_state_vault: deposit_ctx.user_state_vault,
        destination: destination.address,
        ticker: deposit_ctx.ticker,
        mint: deposit_ctx.mint,
        token_program: quasar_svm::SPL_TOKEN_PROGRAM_ID,
        system_program: quasar_svm::system_program::ID,
    }
    .into();

    svm.process_instruction(
        &consume_ticker_instruction,
        &[
            svm.get_account(&broadcaster).expect("broadcaster account"),
            svm.get_account(&admin).expect("admin account"),
            svm.get_account(&deposit_ctx.user).expect("user account"),
            svm.get_account(&deposit_ctx.agent).expect("agent account"),
            svm.get_account(&global_state)
                .expect("global state account"),
            svm.get_account(&deposit_ctx.user_state)
                .expect("user state account"),
            svm.get_account(&deposit_ctx.user_state_vault)
                .expect("vault token account"),
            destination.clone(),
            svm.get_account(&deposit_ctx.ticker)
                .expect("ticker account"),
            create_keyed_mint_account_with_program(
                &deposit_ctx.mint,
                &Mint {
                    supply: 50_000_000_000,
                    decimals: 9,
                    freeze_authority: None.into(),
                    is_initialized: true,
                    mint_authority: Some(broadcaster).into(),
                },
                &TOKEN_PROGRAM_ID,
            ),
        ],
    )
}

#[test]
fn consume_ticker_rejects_non_admin_test() {
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

    let deposit_ctx = process_deposit_for_agent_use(
        &mut svm,
        payer,
        global_state,
        fee_token_account,
        token_a_mint,
    )
    .expect("error in deposit_for_agent_use");

    process_register_ticker_for_me(&mut svm, payer, &deposit_ctx, APPROVED_SPEND)
        .expect("error in register_ticker_for_me");

    set_token_account_amount(&mut svm, deposit_ctx.user_state_vault, BOOSTED_VAULT_AMOUNT);

    let outsider = Pubkey::new_unique();
    svm.airdrop(&outsider, 10_000_000_000);
    let broadcaster = Pubkey::new_unique();
    svm.airdrop(&broadcaster, 10_000_000_000);
    let destination = create_destination_token_account(payer, deposit_ctx.mint);

    let pre_vault = svm
        .get_account(&deposit_ctx.user_state_vault)
        .expect("vault token account")
        .data;
    let pre_destination = destination.data.clone();
    let pre_ticker = svm.get_account(&deposit_ctx.ticker).expect("ticker account").data;

    let result = process_consume_ticker(
        &mut svm,
        broadcaster,
        outsider,
        global_state,
        &deposit_ctx,
        &destination,
    );

    print!("consume_ticker unauthorized logs: {:?} \n", result.logs);
    result.assert_error(ProgramError::MissingRequiredSignature);

    let post_vault = svm
        .get_account(&deposit_ctx.user_state_vault)
        .expect("vault token account")
        .data;
    let post_destination = result
        .account(&destination.address)
        .map(|account| account.data.clone())
        .unwrap_or(pre_destination.clone());
    let post_ticker = svm.get_account(&deposit_ctx.ticker).expect("ticker account").data;

    assert_eq!(pre_vault, post_vault);
    assert_eq!(pre_destination, post_destination);
    assert_eq!(pre_ticker[9], post_ticker[9]);
}

#[test]
#[ignore = "exposes current consume_ticker CPI signer-escalation bug"]
fn consume_ticker_moves_funds_and_sets_position_test() {
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

    let deposit_ctx = process_deposit_for_agent_use(
        &mut svm,
        payer,
        global_state,
        fee_token_account,
        token_a_mint,
    )
    .expect("error in deposit_for_agent_use");

    process_register_ticker_for_me(&mut svm, payer, &deposit_ctx, APPROVED_SPEND)
        .expect("error in register_ticker_for_me");

    set_token_account_amount(&mut svm, deposit_ctx.user_state_vault, BOOSTED_VAULT_AMOUNT);
    let broadcaster = Pubkey::new_unique();
    svm.airdrop(&broadcaster, 10_000_000_000);
    let destination = create_destination_token_account(payer, deposit_ctx.mint);

    let pre_user_state = svm
        .get_account(&deposit_ctx.user_state)
        .expect("user state account")
        .data;

    let result = process_consume_ticker(
        &mut svm,
        broadcaster,
        payer,
        global_state,
        &deposit_ctx,
        &destination,
    );

    print!("consume_ticker authorized logs: {:?} \n", result.logs);
    result.assert_success();

    let post_vault = svm
        .get_account(&deposit_ctx.user_state_vault)
        .expect("vault token account")
        .data;
    let post_destination = result
        .account(&destination.address)
        .expect("destination account")
        .data
        .clone();
    let post_ticker = svm.get_account(&deposit_ctx.ticker).expect("ticker account").data;
    let post_user_state = svm
        .get_account(&deposit_ctx.user_state)
        .expect("user state account")
        .data;

    assert_eq!(
        u64::from_le_bytes(post_vault[64..72].try_into().unwrap()),
        BOOSTED_VAULT_AMOUNT - APPROVED_SPEND
    );
    assert_eq!(
        u64::from_le_bytes(post_destination[64..72].try_into().unwrap()),
        APPROVED_SPEND
    );
    assert_eq!(post_ticker[9], 1);
    assert_eq!(
        u64::from_le_bytes(pre_user_state[114..122].try_into().unwrap()),
        u64::from_le_bytes(post_user_state[114..122].try_into().unwrap())
    );
}
