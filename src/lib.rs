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
    pub fn delete_agent(ctx: Ctx<Agent>, name: PodString<20>) -> Result<(), ProgramError> {
        ctx.accounts.register_agent(name)
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
}

#[cfg(test)]
mod tests;
