# Build Context

review:
  date: 2026-07-23
  security_score: C
  quality_score: B
  ready_for_mainnet: false
  findings:
    - severity: High
      category: Security
      description: Deleting an agent can strand user funds because user vault flows depend on a live `IAgent` account.
      fix: Prevent `delete_agent` while dependent `UserState` or `Ticker` accounts exist, or redesign downstream PDAs to depend on immutable `agent_id` instead of the live agent account address.
    - severity: High
      category: Security
      description: `initialize` is re-runnable by the hard-coded deployer and can overwrite admins, fees, and fee destination after launch.
      fix: Add a one-time initialization guard or a separate admin-governed config update instruction.
    - severity: Medium
      category: Security
      description: `update_ticker_close_trade` is permissionless, so any caller can clear `is_in_position` for any known user-agent ticker.
      fix: Require an authorized signer or only allow the flag to change through a validated trade-close path.
    - severity: Medium
      category: Correctness
      description: `consume_ticker` uses unchecked decimal exponentiation and assertion panics, which can turn malformed mint configuration into instruction aborts.
      fix: Replace `assert!` and raw `pow` with checked math and explicit program errors.
    - severity: Low
      category: Correctness
      description: `consume_ticker` signs with the bump stored in `UserState` without re-deriving the canonical PDA on the hot path.
      fix: Re-derive the PDA and canonical bump before `invoke_signed`, or assert that the stored bump matches the derived bump.
