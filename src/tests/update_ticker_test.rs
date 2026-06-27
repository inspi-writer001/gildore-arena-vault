use crate::tests::{
    create_ticker::{process_register_ticker_for_me, process_update_ticker_close_trade},
    deposit_test::process_deposit_for_agent_use,
    initialize::process_initialize,
    register_agent::process_register_agent,
    setup, ReusableState,
};
use quasar_svm::Account;

const TEST_AGENT_NAME: &str = "alpha-bot";
const APPROVED_SPEND: u64 = 50_000;

fn set_ticker_in_position(svm: &mut quasar_svm::QuasarSvm, ticker: solana_address::Address) {
    let mut ticker_account: Account = svm.get_account(&ticker).expect("ticker account");
    ticker_account.data[9] = 1;
    svm.set_account(ticker_account);
}

#[test]
fn update_ticker_close_trade_clears_in_position_flag_test() {
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

    set_ticker_in_position(&mut svm, deposit_ctx.ticker);

    process_update_ticker_close_trade(&mut svm, payer, &deposit_ctx)
        .expect("error in update_ticker_close_trade");

    let ticker_account = svm
        .get_account(&deposit_ctx.ticker)
        .expect("ticker should exist");
    assert_eq!(ticker_account.data[9], 0);
}
