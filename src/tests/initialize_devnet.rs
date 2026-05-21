use std::{env, error::Error, str::FromStr};

use crate::{
    cpi::InitializeInstruction,
    instructions::InitializeArgs,
    state::{DEPLOYER_ADDRESS, GLOBAL_STATE_SEED},
};
use quasar_lang::Vec;
use solana_address::Address;
use solana_commitment_config::CommitmentConfig;
use solana_keypair::{read_keypair_file, Keypair, Signer};
use solana_pubkey::Pubkey;
use solana_rpc_client::rpc_client::RpcClient;
use solana_transaction::Transaction;
// use zeropod::Vec;

const DEFAULT_DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";

#[test]
#[ignore = "hits Devnet RPC and requires funded env-configured deployer keypair"]
fn test_initialize_devnet() {
    process_initialize_devnet().expect("devnet initialize should succeed");
}

pub fn process_initialize_devnet() -> Result<(), Box<dyn Error>> {
    let rpc_url = env::var("DEVNET_RPC_URL").unwrap_or_else(|_| DEFAULT_DEVNET_RPC_URL.to_string());
    let payer_keypair_path = env::var("DEVNET_PAYER_KEYPAIR")
        .map_err(|_| "missing DEVNET_PAYER_KEYPAIR env var pointing to the deployer keypair")?;
    let fee_token_account = Address::from_str(
        &env::var("DEVNET_FEE_TOKEN_ACCOUNT")
            .map_err(|_| "missing DEVNET_FEE_TOKEN_ACCOUNT env var")?,
    )?;

    let payer = read_keypair_file(&payer_keypair_path)?;
    assert_eq!(
        payer.pubkey(),
        DEPLOYER_ADDRESS,
        "DEVNET_PAYER_KEYPAIR must correspond to DEPLOYER_ADDRESS"
    );

    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let fee_token_pubkey = Pubkey::from(fee_token_account);
    client.get_account(&fee_token_pubkey)?;

    let (global_state, _) =
        Address::derive_program_address(&[GLOBAL_STATE_SEED.as_ref()], &crate::ID)
            .expect("global_state PDA should derive");

    if client.get_account(&Pubkey::from(global_state)).is_ok() {
        return Err(format!(
            "global state account {} already exists on Devnet",
            global_state
        )
        .into());
    }

    let mut admins: Vec<Address, 4> = Vec::default();
    let _ = admins.push(DEPLOYER_ADDRESS);

    let initialize_instruction =
        Into::<solana_instruction::Instruction>::into(InitializeInstruction {
            args: InitializeArgs {
                fee_bps: 15,
                max_fee: 20,
                admin: admins,
            },
            destination_token_account: fee_token_account,
            global_state_account: global_state,
            payer: payer.pubkey(),
            system_program: quasar_svm::system_program::ID,
        });

    let latest_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[initialize_instruction],
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;

    println!("Devnet initialize signature: {}", signature);
    println!("Devnet global_state PDA: {}", global_state);
    println!("Devnet fee token account: {}", fee_token_account);

    Ok(())
}

#[allow(dead_code)]
fn _assert_keypair_type(_: &Keypair) {}
