pub mod errors;
pub mod state;
pub mod instructions;
pub mod constants;
pub mod utils;

use crate::instructions::*;
use anchor_lang::prelude::*;
use state::Config;

declare_id!("EZZtc7TU4Dd5Bc1wdQZ9szhsv3cavuHzNCy8Laq1beLU");

#[program]
pub mod lending {
    use super::*;

    //  called by admin to set global config
    //  need to check the signer is authority
    pub fn configure(ctx: Context<Configure>, new_config: Config) -> Result<()> {
        ctx.accounts.process(new_config, ctx.bumps.config)
    }

    pub fn create_loan(ctx: Context<CreateLoan>, loan_amount: u64, interest_rate: u64, duration: u64, collateral_amount: u64) -> Result<()> {
        ctx.accounts.process(loan_amount, interest_rate, duration, collateral_amount)
    }

    pub fn borrow_loan(ctx: Context<BorrowLoan>, collateral_amount: u64) -> Result<()> {
        ctx.accounts.process(collateral_amount, ctx.bumps.loan)
    }

    pub fn cancel_loan(ctx: Context<CancelLoan>) -> Result<()> {
        ctx.accounts.process(ctx.bumps.loan)
    }

    pub fn repay_loan(ctx: Context<RepayLoan>) -> Result<()> {
        ctx.accounts.process(ctx.bumps.loan)
    }

}
