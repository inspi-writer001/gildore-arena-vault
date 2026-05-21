use core::fmt::Error;

use crate::{
    cpi::RegisterAgentInstruction,
    hashing::{sol_sha256, SolBytes},
    state::{DEPLOYER_ADDRESS, GLOBAL_STATE_SEED},
    tests::{initialize::process_initialize, setup, ReusableState},
};
use quasar_lang::Vec;
use quasar_svm::{system_program, Account, Instruction};
use sha2::{Digest, Sha256};
use solana_address::Address;

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
#[test]
fn register_agent_test() {
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
    .expect("error in initialize");

    process_register_agent("alpha-bot", &mut svm, global_state, payer)
        .expect("error in register_agent");
}

pub fn process_register_agent(
    name: &str,
    svm: &mut quasar_svm::QuasarSvm,
    global_state: Address,
    payer: Address,
) -> Result<(), Error> {
    let agent_id = derive_agent_id(name);

    println!("Agent ID for alpha-bot: {:?}", agent_id);

    let expected_hash: [u8; 32] =
        Sha256::digest([crate::ID.as_ref(), name.as_bytes()].concat()).into();
    assert_eq!(agent_id.as_ref(), &expected_hash);

    let (agent_address, bump) =
        Address::derive_program_address(&[b"agent", agent_id.as_ref()], &crate::ID)
            .expect("agent PDA should derive");
    println!("Address for Agent: {:?}", agent_address);

    let saved_seeds: [u8; 37] = [b"agent", agent_id.as_ref()]
        .concat()
        .try_into()
        .expect("seed data should be 37 bytes");

    assert_ne!(agent_address, Address::default());
    assert!(
        u8::try_from(bump).is_ok(),
        "bump should fit in a single byte"
    );
    assert_eq!(&saved_seeds[..5], b"agent");
    assert_eq!(&saved_seeds[5..], agent_id.as_ref());

    let register_agent_instruction_: Instruction = RegisterAgentInstruction {
        admin: payer,
        agent: agent_address,
        global_state_account: global_state,
        name: name.into(),
        system_program: system_program::ID,
    }
    .into();

    let payer_account = svm.get_account(&payer).unwrap();

    let global_state_account = svm.get_account(&global_state).unwrap();

    let result = svm.process_instruction(
        &register_agent_instruction_,
        &[
            payer_account,
            Account {
                address: agent_address,
                lamports: 0,
                data: vec![],
                executable: false,
                owner: system_program::ID,
            },
            global_state_account,
        ],
    );

    print!("create_agent CUs: {:?} \n", result.compute_units_consumed);

    print!("create_agent logs: {:?} \n", result.logs);
    result.assert_success();
    Ok(())
}
