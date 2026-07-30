use quasar_lang::{prelude::*, sysvars::Sysvar};
use quasar_spl::prelude::*;
use quasar_spl::{Mint, Token, TokenCpi, TokenInterface, TokenProgram};

use crate::state::{GlobalState, IAgent, Ticker, UserState, UserStateInner};

#[derive(Accounts)]
pub struct UserDepositForAgentUse {
    #[account(mut)]
    pub payer: Signer,

    #[account(mut)]
    pub user: Signer,

    #[account(
        mut,
        address= IAgent::seeds(agent.agent_id())
    )]
    pub agent: Account<IAgent>,

    #[account(
        mut,
       address = GlobalState::seeds(),
    )]
    pub global_state_account: Account<GlobalState>,

    #[account(
        init(idempotent),
        payer = payer,
       address = UserState::seeds(user.address(),  mint.address(), agent.address()),
    )]
    pub user_state: Account<UserState>,

    #[account(
        init(idempotent),
        payer = payer,
        token(
            mint = mint, authority = user_state, token_program = token_program
        ),
    )]
    pub user_state_vault: InterfaceAccount<Token>,

    #[account(
        init(idempotent),
        payer = payer,
        address = Ticker::seeds(agent.agent_id(), user.address())
    )]
    pub ticker: Account<Ticker>,

    #[account(mut, address= global_state_account.fee_destination)]
    pub destination_fee_token_account: InterfaceAccount<Token>,

    #[account(mut)]
    pub user_token_account: InterfaceAccount<Token>,

    #[account()]
    pub mint: InterfaceAccount<Mint>,

    pub token_program: Interface<TokenInterface>,
    // pub token_program: Program<TokenProgram>,
    pub system_program: Program<SystemProgram>,
}

impl UserDepositForAgentUse {
    #[inline(always)]
    pub fn deposit_for_agent_use(
        &mut self,
        amount: u64,
        bumps: UserDepositForAgentUseBumps,
    ) -> Result<(), ProgramError> {
        let platform_fee = self.global_state_account.fee_bps;
        // log("hello");
        // self.user_state.user_address.log();
        let amount_to_platform: u64 = {
            // fee_to_deduct =  amount * platform_fee / 10_000

            let to_deduct = compute_platform_fee(
                amount,
                platform_fee.into(),
                self.global_state_account.max_fee.into(),
            )
            .expect("failed_to_calculate_fee");
            // let to_deduct = amount * (u16::from(platform_fee) as u64) / 10_000;

            // if to_deduct >= self.global_state_account.max_fee {
            //     self.global_state_account.max_fee.into()
            // } else {
            //     to_deduct.into()
            // }

            to_deduct
        };

        let amount_to_user_vault = amount - amount_to_platform;

        self.token_program
            .transfer_checked(
                self.user_token_account.to_account_view(),
                self.mint.to_account_view(),
                self.destination_fee_token_account.to_account_view(),
                self.user.to_account_view(),
                amount_to_platform,
                self.mint.decimals(),
            )
            .invoke()?;

        self.token_program
            .transfer_checked(
                self.user_token_account.to_account_view(),
                self.mint.to_account_view(),
                self.user_state_vault.to_account_view(),
                self.user.to_account_view(),
                amount_to_user_vault,
                self.mint.decimals(),
            )
            .invoke()?;

        let current_time = i64::from(Clock::get()?.unix_timestamp) as u64;

        if self.user_state.is_initialized == false {
            self.user_state.set_inner(UserStateInner {
                agent_id: self.agent.agent_id,
                net_deposited_amount: amount_to_user_vault,
                is_initialized: true.into(),
                bump: bumps.user_state,
                created_time: current_time,
                modified_time: current_time,
                user_address: *self.user.address(),
                ticker_id: *self.ticker.address(),
            })
        } else {
            let previous_amount = self.user_state.net_deposited_amount();

            let new_amount = previous_amount.saturating_add(amount_to_user_vault);

            let current_time = i64::from(Clock::get()?.unix_timestamp) as u64;

            self.user_state.modified_time = current_time.into();
            self.user_state.net_deposited_amount = new_amount.into();
        }
        Ok(())
    }
}

#[inline(always)]
pub fn compute_platform_fee(amount: u64, fee_bps: u16, max_fee: u64) -> Result<u64, ProgramError> {
    let raw_fee_u128 = (amount as u128)
        .checked_mul(fee_bps as u128)
        .expect("multiplication failed")
        / 10_000u128;

    let capped_fee_u128 = raw_fee_u128.min(max_fee as u128);

    let fee = u64::try_from(capped_fee_u128).expect("failed_to_cast_to_u64");

    if fee > amount {
        return Err(ProgramError::InvalidArgument);
    }

    Ok(fee)
}
