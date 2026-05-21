use core::fmt::Error;

use crate::{
    cpi::InitializeInstruction,
    instructions::InitializeArgs,
    state::{IAgent, DEPLOYER_ADDRESS},
    tests::{setup, ReusableState},
};
use quasar_lang::{
    account_layout::AccountLayout,
    traits::{Discriminator, Space},
};
use spl_token_interface::{state::Account as TokenAccount, state::AccountState};

use quasar_svm::{system_program, Account, Instruction};
use solana_address::Address;
use zeropod::Vec;

#[test]
fn test_initialize() {
    let (mut svm, reusable_state) = setup();

    let ReusableState {
        fee_token_account,
        global_state,
        payer,
        token_a_address,
        token_a_mint,
    } = reusable_state;

    process_initialize(
        &mut svm,
        fee_token_account,
        global_state,
        token_a_mint,
        payer,
    )
    .unwrap();

    let bump_offsef = IAgent::BUMP_OFFSET;
    let data_offset = IAgent::DATA_OFFSET;
    let data_size = IAgent::DATA_SIZE;
    let space = IAgent::SPACE;

    println!(
        "{:?}\n{:?}\n{:?}\n{:?}",
        bump_offsef, data_offset, data_size, space
    );

    // let instruction = initialize_instruction(
    //     payer,
    //     global_state,
    //     fee_token_account,
    //     Address::from(quasar_svm::system_program::ID.to_bytes()),
    // );
}

pub fn process_initialize(
    svm: &mut quasar_svm::QuasarSvm,
    fee_token_account: Address,
    global_state: Address,
    token_a_mint: Address,
    payer: Address,
) -> Result<(), Error> {
    svm.airdrop(&payer, 10_000_000_000);
    let payer_account = svm.get_account(&payer).unwrap();

    let token_account = TokenAccount {
        mint: token_a_mint,
        owner: payer,
        state: AccountState::Initialized,
        ..Default::default()
    };

    let keyed_token_account =
        quasar_svm::token::create_keyed_token_account(&fee_token_account, &token_account);

    let mut admins: Vec<Address, 4> = Vec::<Address, 4>::default();
    let _ = admins.push(DEPLOYER_ADDRESS);

    let initialize_instruction_: Instruction = InitializeInstruction {
        args: InitializeArgs {
            fee_bps: 10,
            max_fee: 20,
            admin: admins,
        },
        destination_token_account: fee_token_account,
        global_state_account: global_state,
        payer,
        system_program: system_program::ID,
    }
    .into();

    let result = svm.process_instruction(
        &initialize_instruction_,
        &[
            payer_account,
            Account {
                address: global_state,
                lamports: 0,
                // data: vec![0; core::mem::size_of::<GlobalStateData>()],
                owner: system_program::ID, // needs to be system program because we need to create it
                executable: false,
                data: vec![],
            },
            keyed_token_account,
        ],
    );

    print!("initialize CUs: {:?} \n", result.compute_units_consumed);
    result.assert_success();

    Ok(())
}
