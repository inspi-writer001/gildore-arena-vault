# Gildore Vault

This program is a Quasar-based Solana vault for agent-managed trading.

The short version is:

- Admins register agents.
- Users deposit into a per-agent vault.
- The program keeps accounting in `UserState`.
- A user can set a ticker-level spend limit.
- An admin-approved consume flow can move tokens out of the vault into a trading destination.
- When the trade is considered closed, the ticker flag is cleared.
- Users withdraw from the vault, and only deposit and withdrawal mutate the tracked net deposited amount.

If you already understand an Anchor vault, the easiest mental model is:

- `GlobalState` is the protocol config PDA.
- `IAgent` is the registered agent PDA.
- `UserState` is the per-user, per-agent, per-mint accounting PDA.
- `user_state_vault` is the token account owned by `UserState`.
- `Ticker` is a lightweight PDA that stores how much the user is willing to let the agent spend and whether that ticker is currently marked as in position.

## Repo Layout

- [src/lib.rs](/home/inspiration-gx/Documents/gildore-project/gildore-vault/src/lib.rs) wires the program entrypoints.
- [src/state.rs](/home/inspiration-gx/Documents/gildore-project/gildore-vault/src/state.rs) defines the main PDA-backed account types.
- [src/instructions/initialize.rs](/home/inspiration-gx/Documents/gildore-project/gildore-vault/src/instructions/initialize.rs) creates protocol-wide config.
- [src/instructions/register_agent.rs](/home/inspiration-gx/Documents/gildore-project/gildore-vault/src/instructions/register_agent.rs) registers and deletes agents.
- [src/instructions/deposit.rs](/home/inspiration-gx/Documents/gildore-project/gildore-vault/src/instructions/deposit.rs) handles deposits and fee collection.
- [src/instructions/ticker.rs](/home/inspiration-gx/Documents/gildore-project/gildore-vault/src/instructions/ticker.rs) stores the user-approved spend amount.
- [src/instructions/consume_ticker.rs](/home/inspiration-gx/Documents/gildore-project/gildore-vault/src/instructions/consume_ticker.rs) moves funds from the vault into a trade destination and marks the ticker in position.
- [src/instructions/user_withdraw.rs](/home/inspiration-gx/Documents/gildore-project/gildore-vault/src/instructions/user_withdraw.rs) lets the user withdraw from their vault and updates accounting.
- [src/tests](/home/inspiration-gx/Documents/gildore-project/gildore-vault/src/tests) shows the intended end-to-end flow in small pieces.

## The Main Accounts

### `GlobalState`

`GlobalState` is derived from the static seed `b"global_state"`.

It stores:

- fee destination token account
- fee basis points
- max fee cap
- admin list

This is created once through `initialize`.

### `IAgent`

`IAgent` is derived from `b"agent"` plus a deterministic `agent_id`.

That `agent_id` is not random. It is the SHA-256 of:

- the program id
- the agent name bytes

That means the same program id plus the same name gives the same agent id every time. The program then derives the agent PDA from that id and stores the seed material on the account.

### `UserState`

`UserState` is the core accounting account.

Seeds:

- `b"user_state"`
- `user`
- `mint`
- `agent`

It stores:

- the user address
- the agent id
- the ticker PDA address
- timestamps
- a boolean init flag
- `net_deposited_amount`
- the PDA bump

The important point is that `net_deposited_amount` is not "current vault token balance". It is accounting for net user deposits after fees and withdrawals.

That distinction matters because the vault tokens can be temporarily moved out for trading. The token account balance can change during trading, but the tracked deposit accounting should only move when the user deposits or withdraws.

### `Ticker`

`Ticker` is derived from:

- `b"ticker"`
- `agent_id`
- `user`

It stores:

- `amount_to_spend`
- `is_in_position`

This is a control/accounting helper for trading, not the source of truth for user funds.

## Instruction Walkthrough

### 1. `initialize`

`initialize` creates `GlobalState` and writes:

- protocol fee config
- max fee
- fee destination
- admin addresses

The payer is locked to `DEPLOYER_ADDRESS` in the current code, so this is not a permissionless setup instruction.

### 2. `register_agent`

`register_agent` can only be called by an address that exists in `GlobalState.admin`.

Flow:

1. Hash `program_id || name` to get `agent_id`.
2. Derive the `IAgent` PDA from `b"agent"` and `agent_id`.
3. Create the PDA account with `invoke_signed`.
4. Write the discriminator, agent id, bump, and saved seeds directly into the account data.

This file is worth reading carefully because it is the clearest example in the repo of Quasar PDA creation without Anchor sugar.

### 3. `deposit_for_agent_use`

This is where the user joins an agent vault.

It does three things in one instruction:

1. Creates or reuses the `UserState` PDA.
2. Creates or reuses the token vault owned by `UserState`.
3. Creates or reuses the `Ticker` PDA.

Then it transfers tokens:

- one transfer from the user token account to the protocol fee destination
- one transfer from the user token account to the user vault

After that, it updates `UserState.net_deposited_amount`.

Current meaning of `net_deposited_amount`:

- deposit increases it by the amount that actually entered the user vault
- withdrawal decreases it by the amount the user removed
- trading does not change it

That is the cleanest mental model in this codebase right now.

### 4. `register_ticker_for_me`

This instruction is intentionally simple.

It sets:

- `ticker.amount_to_spend`

It does not:

- move tokens
- change `UserState`
- clear `is_in_position`

That last point is deliberate. Re-registering a ticker updates the approved spend amount, but it does not pretend an open trade has disappeared.

### 5. `consume_ticker`

This is the trading handoff instruction.

Accounts include:

- a `broadcaster` signer, so someone else can pay for the transaction
- an `admin` signer, so the movement of vault funds still requires trusted approval
- the `UserState` PDA
- the token vault owned by `UserState`
- a destination token account
- the `Ticker`

Flow:

1. Check that `admin` is in `GlobalState.admin`.
2. Read the vault token balance.
3. Read `ticker.amount_to_spend`.
4. Enforce a minimum vault size before trading.
5. Decide the transfer amount:
   - if ticker amount is zero, use either the hard cap or the full vault amount
   - if ticker amount is above the vault balance, clamp it to the vault balance
6. Sign as the `UserState` PDA and transfer from the vault to the destination account.
7. Mark `ticker.is_in_position = true`.

The most important design choice here is what does not happen:

- `UserState.net_deposited_amount` is not reduced

That means the tracked deposited amount remains the reference number, while the live token vault balance shows what is actually still sitting in the vault. The difference between those two values is what a client can use to reason about deployed funds, profit, or loss.

### 6. `update_ticker_close_trade`

This is the close marker.

It currently does one thing:

- set `ticker.is_in_position = false`

It does not move tokens back by itself. If the trading system has already transferred tokens back into the vault token account, this instruction just updates program state so the ticker no longer reads as currently in position.

### 7. `user_withdrawal`

The user withdraws directly from the `user_state_vault`.

Flow:

1. Check the vault token balance is enough.
2. Re-derive the expected `UserState` PDA.
3. Sign as the `UserState` PDA.
4. Transfer vault tokens back to the user token account.
5. Decrease `UserState.net_deposited_amount`.
6. Update `modified_time`.

This is the other instruction, alongside deposit, that changes the tracked net deposited amount.

## How the Accounting Works

This is the part a junior engineer should keep straight:

- `user_state_vault.amount()` is the live SPL token balance in the vault token account.
- `user_state.net_deposited_amount()` is the program's accounting of net deposits after fees and withdrawals.

They are not always supposed to match.

They match when:

- no trade is active
- no profit or loss has changed the returned balance

They can differ when:

- the agent has consumed funds into an active trade
- a trade closed with profit
- a trade closed with loss

That is why `consume_ticker` should not reduce `net_deposited_amount`. If it did, trading activity would be mixed up with user deposit accounting, and the state would become harder to reason about.

## Security Notes

A few security decisions in the current code are worth understanding before changing anything:

- `register_agent` is admin-gated through `GlobalState.admin`.
- `delete_agent` is also admin-gated and re-derives the canonical PDA before closing.
- `consume_ticker` is admin-gated even though it supports a separate fee-paying `broadcaster`.
- `update_ticker_close_trade` is currently permissionless apart from requiring the correct accounts. That fits a design where anyone can submit the close marker transaction after a trade has already been unwound off-program.

If you tighten or loosen any of these rules later, update both the code and the tests together.

## Tests to Read First

If you want the fastest way to understand the program, read the tests in this order:

1. [initialize.rs](/home/inspiration-gx/Documents/gildore-project/gildore-vault/src/tests/initialize.rs)
2. [register_agent.rs](/home/inspiration-gx/Documents/gildore-project/gildore-vault/src/tests/register_agent.rs)
3. [deposit_test.rs](/home/inspiration-gx/Documents/gildore-project/gildore-vault/src/tests/deposit_test.rs)
4. [create_ticker.rs](/home/inspiration-gx/Documents/gildore-project/gildore-vault/src/tests/create_ticker.rs)
5. [consume_ticker_test.rs](/home/inspiration-gx/Documents/gildore-project/gildore-vault/src/tests/consume_ticker_test.rs)
6. [update_ticker_test.rs](/home/inspiration-gx/Documents/gildore-project/gildore-vault/src/tests/update_ticker_test.rs)

That sequence matches the actual lifecycle better than reading the files alphabetically.

## Build and Test

Common commands:

```bash
quasar build
cargo test -q
```

If you want the full Rust test module path configured in `Quasar.toml`, use:

```bash
cargo test tests::
```

## Running Devnet tests

```bash
set -a
source .env
set +a
cargo test initialize_devnet -- --nocapture

```
