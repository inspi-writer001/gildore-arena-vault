use core::fmt::Error;

use crate::{
    cpi::RegisterTickerForMeInstruction,
    tests::{
        deposit_test::{process_deposit_for_agent_use, DepositContext},
        initialize::process_initialize,
        register_agent::process_register_agent,
        setup, ReusableState,
    },
};
use quasar_svm::{system_program, Instruction};
use spl_token_interface::{state::Mint, ID as TOKEN_PROGRAM_ID};

const TEST_AGENT_NAME: &str = "alpha-bot";
const APPROVED_SPEND: u64 = 50_000;

// DISCLAIMER: this particular test was written by AI referencing from register_agent.rs

fn process_register_ticker_for_me(
    svm: &mut quasar_svm::QuasarSvm,
    payer: solana_address::Address,
    deposit_ctx: &DepositContext,
) -> Result<(), Error> {
    let register_ticker_instruction: Instruction = RegisterTickerForMeInstruction {
        payer,
        user: deposit_ctx.user,
        agent: deposit_ctx.agent,
        user_state: deposit_ctx.user_state,
        user_state_vault: deposit_ctx.user_state_vault,
        ticker: deposit_ctx.ticker,
        mint: deposit_ctx.mint,
        token_program: quasar_svm::SPL_TOKEN_PROGRAM_ID,
        system_program: system_program::ID,
        amount_to_spend: APPROVED_SPEND,
    }
    .into();

    let result = svm.process_instruction(
        &register_ticker_instruction,
        &[
            svm.get_account(&payer).expect("payer account"),
            svm.get_account(&deposit_ctx.user).expect("user account"),
            svm.get_account(&deposit_ctx.agent).expect("agent account"),
            svm.get_account(&deposit_ctx.user_state)
                .expect("user state account"),
            svm.get_account(&deposit_ctx.user_state_vault)
                .expect("vault token account"),
            svm.get_account(&deposit_ctx.ticker)
                .expect("ticker account"),
            quasar_svm::token::create_keyed_mint_account_with_program(
                &deposit_ctx.mint,
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

    print!(
        "register_ticker_for_me CUs: {:?} \n",
        result.compute_units_consumed
    );
    print!("register_ticker_for_me logs: {:?} \n", result.logs);
    result.assert_success();

    Ok(())
}

#[test]
fn create_ticker_test() {
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

    process_register_ticker_for_me(&mut svm, payer, &deposit_ctx)
        .expect("error in register_ticker_for_me");

    let ticker_account = svm
        .get_account(&deposit_ctx.ticker)
        .expect("ticker should exist");
    assert_eq!(ticker_account.owner, crate::ID);
    assert_eq!(ticker_account.data[0], 5);
    assert_eq!(
        u64::from_le_bytes(ticker_account.data[1..9].try_into().unwrap()),
        APPROVED_SPEND
    );
}
