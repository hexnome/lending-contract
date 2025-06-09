use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub authority: Pubkey,

    pub pending_authority: Pubkey,

    pub team_wallet: Pubkey,

    pub lend_fee: u64,
    pub borrow_fee: u64,

    pub expire_duration: u8,
}

#[account]
pub struct Loan{
    pub lender: Pubkey,
    pub borrower: Pubkey,
    
    pub loan_mint: Pubkey,
    pub loan_amount: u64,
    pub interest_rate: u64,
    pub duration: i64,
    pub collateral_mint: Pubkey,
    pub collateral_amount: u64,

    pub create_date: i64,
    pub expire_date: i64,
    pub borrow_date: i64,
    pub repaid: bool,
}