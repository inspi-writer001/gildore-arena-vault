#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod errors;
mod hashing;
mod instructions;
mod state;
use instructions::*;

declare_id!("2um3F4vyQwcuhwrGdPHGMwwK5C4K5rFU84cxHPoYNMKg");

#[program]
mod gildore_vault {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn initialize(ctx: Ctx<Initialize>, args: InitializeArgs) -> Result<(), ProgramError> {
        ctx.accounts.initialize(args, ctx.bumps)
    }

    #[instruction(discriminator = 1)]
    pub fn register_agent(ctx: Ctx<Agent>, name: PodString<20>) -> Result<(), ProgramError> {
        ctx.accounts.register_agent(name)
    }

    #[instruction(discriminator = 2)]
    pub fn delete_agent(ctx: Ctx<Agent>, id: Address) -> Result<(), ProgramError> {
        ctx.accounts.delete_agent(id)
    }

    #[instruction(discriminator = 3)]
    pub fn deposit_for_agent_use(
        ctx: Ctx<UserDepositForAgentUse>,
        amount: u64,
    ) -> Result<(), ProgramError> {
        ctx.accounts.deposit_for_agent_use(amount, ctx.bumps)
    }

    #[instruction(discriminator = 4)]
    pub fn register_ticker_for_me(
        ctx: Ctx<ATicker>,
        amount_to_spend: u64,
    ) -> Result<(), ProgramError> {
        ctx.accounts.register_ticker_for_me(amount_to_spend)
    }

    #[instruction(discriminator = 5)]
    pub fn consume_ticker(ctx: Ctx<ConsumeTickerForUser>) -> Result<(), ProgramError> {
        ctx.accounts.consume_ticker_for_user()
    }

    #[instruction(discriminator = 6)]
    pub fn user_withdrawal(
        ctx: Ctx<UserWithdrawFromParticularAgentVault>,
        amount: u64,
    ) -> Result<(), ProgramError> {
        ctx.accounts.withdraw(amount)
    }

    #[instruction(discriminator = 7)]
    pub fn update_ticker_close_trade(ctx: Ctx<CloseTrade>) -> Result<(), ProgramError> {
        ctx.accounts.update_ticker_close_trade()
    }
}

#[cfg(test)]
mod tests;
