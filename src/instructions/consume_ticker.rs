use quasar_lang::{cpi::Seed, prelude::*};

use quasar_spl::prelude::*;

use crate::state::{GlobalState, IAgent, Ticker, UserState};

const HARD_CAP_SPENDABLE: u64 = 30;
const MINIMUM_SPENDABLE: u64 = 15;

#[derive(Accounts)]
// #[instruction(user: Address)]
pub struct ConsumeTickerForUser {
    #[account(mut)]
    pub broadcaster: Signer,

    #[account()]
    pub user: SystemAccount,

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
       mut,
       address = UserState::seeds(user.address(),  mint.address(), agent.address()),
    )]
    pub user_state: Account<UserState>,

    #[account(
        mut,
       constraints(&user_state_vault.mint == mint.address()),
       constraints(&user_state_vault.owner == user_state.address())
    )]
    pub user_state_vault: InterfaceAccount<Token>,

    #[account(mut)]
    pub destination: InterfaceAccount<Token>,

    #[account(
        mut,
        address = Ticker::seeds(agent.agent_id(), user.address())
    )]
    pub ticker: Account<Ticker>,

    #[account()]
    pub mint: InterfaceAccount<Mint>,

    pub token_program: Interface<TokenInterface>,
    // pub token_program: Program<TokenProgram>,
    pub system_program: Program<SystemProgram>,
}

impl ConsumeTickerForUser {
    pub fn consume_ticker_for_user(&mut self) -> Result<(), ProgramError> {
        let mut spendable_amount = self.ticker.amount_to_spend();
        let normal_minimum_spendable = MINIMUM_SPENDABLE * 10u64.pow(self.mint.decimals() as u32);
        let normal_hard_cap_spendable = HARD_CAP_SPENDABLE * 10u64.pow(self.mint.decimals() as u32);

        assert!(
            self.user_state_vault.amount() >= normal_minimum_spendable,
            "balance below MINIMUM_SPENDABLE"
        );

        if spendable_amount == 0 {
            if self.user_state_vault.amount() >= normal_hard_cap_spendable {
                spendable_amount = normal_hard_cap_spendable;
            } else {
                spendable_amount = self.user_state_vault.amount();
            }
        }

        let bump = self.user_state.bump.to_le_bytes();

        let signer_seeds = [
            Seed::from(self.user.address().as_ref()),
            Seed::from(self.mint.address().as_ref()),
            Seed::from(self.agent.address().as_ref()),
            Seed::from(&bump),
        ];

        self.token_program
            .transfer_checked(
                &self.user_state_vault,
                &self.mint,
                &self.destination,
                &self.user_state,
                spendable_amount,
                self.mint.decimals(),
            )
            .invoke_signed(&signer_seeds)?;
        Ok(())
    }
}
