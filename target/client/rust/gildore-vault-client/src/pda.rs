use solana_address::Address;

/// Seeds: [b"global_state"]
pub fn find_global_state_account_address(program_id: &Address) -> (Address, u8) {
    Address::find_program_address(&[b"global_state"], program_id)
}

/// Seeds: [b"agent"]
pub fn find_agent_address(program_id: &Address) -> (Address, u8) {
    Address::find_program_address(&[b"agent"], program_id)
}

/// Seeds: [b"user_state"]
pub fn find_user_state_address(program_id: &Address) -> (Address, u8) {
    Address::find_program_address(&[b"user_state"], program_id)
}

/// Seeds: [b"ticker"]
pub fn find_ticker_address(program_id: &Address) -> (Address, u8) {
    Address::find_program_address(&[b"ticker"], program_id)
}

