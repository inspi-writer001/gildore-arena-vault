use std::{env, error::Error};

use crate::{
    cpi::RegisterAgentInstruction,
    hashing::{sol_sha256, SolBytes},
    state::{DEPLOYER_ADDRESS, GLOBAL_STATE_SEED},
};
use solana_address::Address;
use solana_commitment_config::CommitmentConfig;
use solana_keypair::{read_keypair_file, Signer};
use solana_pubkey::Pubkey;
use solana_rpc_client::rpc_client::RpcClient;
use solana_transaction::Transaction;

const DEFAULT_DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";
const AGENT_NAME: &str = "kairos";

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
#[ignore = "hits Devnet RPC and requires funded env-configured deployer keypair plus initialized global state"]
fn test_register_agent_devnet() {
    dotenvy::dotenv().expect("Failed to load .env file");
    process_register_agent_devnet().expect("devnet register_agent should succeed");
}

pub fn process_register_agent_devnet() -> Result<(), Box<dyn Error>> {
    let rpc_url = env::var("DEVNET_RPC_URL").unwrap_or_else(|_| DEFAULT_DEVNET_RPC_URL.to_string());
    let payer_keypair_path = env::var("DEVNET_PAYER_KEYPAIR")
        .map_err(|_| "missing DEVNET_PAYER_KEYPAIR env var pointing to the deployer keypair")?;

    let payer = read_keypair_file(&payer_keypair_path)?;
    assert_eq!(
        payer.pubkey(),
        DEPLOYER_ADDRESS,
        "DEVNET_PAYER_KEYPAIR must correspond to DEPLOYER_ADDRESS"
    );

    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let (global_state, _) =
        Address::derive_program_address(&[GLOBAL_STATE_SEED.as_ref()], &crate::ID)
            .expect("global_state PDA should derive");
    client
        .get_account(&Pubkey::from(global_state))
        .map_err(|_| {
            format!(
                "global state account {} does not exist on Devnet; run initialize first",
                global_state
            )
        })?;

    let agent_id = derive_agent_id(AGENT_NAME);
    let (agent_address, _) =
        Address::derive_program_address(&[b"agent", agent_id.as_ref()], &crate::ID)
            .expect("agent PDA should derive");

    if client.get_account(&Pubkey::from(agent_address)).is_ok() {
        return Err(format!(
            "agent account {} for name '{}' already exists on Devnet",
            agent_address, AGENT_NAME
        )
        .into());
    }

    let register_agent_instruction =
        Into::<solana_instruction::Instruction>::into(RegisterAgentInstruction {
            admin: payer.pubkey(),
            agent: agent_address,
            global_state_account: global_state,
            name: AGENT_NAME.into(),
            system_program: quasar_svm::system_program::ID,
        });

    let latest_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[register_agent_instruction],
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;

    println!("Devnet register_agent signature: {}", signature);
    println!("Devnet global_state PDA: {}", global_state);
    println!("Devnet agent name: {}", AGENT_NAME);
    println!("Devnet agent_id: {}", agent_id);
    println!("Devnet agent PDA: {}", agent_address);

    Ok(())
}
