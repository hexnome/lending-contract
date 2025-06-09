use anchor_lang::prelude::*;
use chrono::Duration;

use crate::{
    constants::{CONFIG, LOAN}, 
    errors::AgioError, 
    state::{Config, Loan}, 
    utils::{ token_transfer_user}
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, Mint},
};

#[derive(Accounts)]
pub struct CreateLoan<'info> {
    #[account(
        mut,
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,

    //  team wallet
    /// CHECK: should be same with the address in the global_config
    #[account(
        mut,
        constraint = global_config.team_wallet == team_wallet.key() @AgioError::IncorrectAuthority
    )]
    pub team_wallet: AccountInfo<'info>,

    #[account(
        init, 
        payer = lender, 
        space = 8 + std::mem::size_of::<Loan>(),
        seeds = [LOAN.as_bytes(), &loan_key.key().to_bytes()],
        bump,
    )]
    loan: Account<'info, Loan>,

    #[account(mut)]
    collateral_mint: Account<'info, Mint>,

    #[account(mut)]
    loan_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [
            loan.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            collateral_mint.key().as_ref(),
        ],
        seeds::program = anchor_spl::associated_token::ID,
        bump,
    )]
    loan_collateral_ata: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            loan.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            loan_mint.key().as_ref(),
        ],
        seeds::program = anchor_spl::associated_token::ID,
        bump
    )]
    loan_mint_ata: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            lender.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            loan_mint.key().as_ref(),
        ],
        bump,
        seeds::program = anchor_spl::associated_token::ID
    )]
    lender_loan_ata: AccountInfo<'info>,

    #[account(mut)]
    pub lender: Signer<'info>,

    pub loan_key: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,

}

impl<'info> CreateLoan<'info>{
    pub fn process(
        &mut self,
        loan_amount: u64,
        interest_rate: u64,
        duration: u64,
        collateral_amount: u64,
    ) -> Result<()> {

        let global_config = &mut self.global_config;
        let loan_mint_ata = &mut self.loan_mint_ata;
        let loan_collateral_ata = &mut self.loan_collateral_ata;
        let collateral_mint = &mut self.collateral_mint;
        let loan_mint = &mut self.loan_mint;
        let lender_loan_ata = &mut self.lender_loan_ata;
        
        let one_day = Duration::days(1).num_seconds();

        // create loan pda
        let loan = &mut self.loan;
        
        loan.lender = self.lender.key();
        loan.borrower = Pubkey::default(); // No lender yet
        
        loan.loan_mint = loan_mint.key();
        loan.loan_amount = loan_amount;
        loan.interest_rate = interest_rate;
        loan.duration = one_day
        .checked_mul(duration as i64)
        .ok_or(AgioError::OverflowOrUnderflowOccurred)?;
        loan.collateral_mint = collateral_mint.key();
        loan.collateral_amount = collateral_amount;
    
        let expire_duration = one_day
            .checked_mul(global_config.expire_duration as i64)
            .ok_or(AgioError::OverflowOrUnderflowOccurred)?;

        loan.create_date = Clock::get()?.unix_timestamp;
        loan.expire_date = loan.create_date
            .checked_add(expire_duration)
            .ok_or(AgioError::OverflowOrUnderflowOccurred)?;
        loan.repaid = false;

        // create loan_mint ata
        anchor_spl::associated_token::create(CpiContext::new(
            self.associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: self.lender.to_account_info(),
                associated_token: loan_mint_ata.to_account_info(),
                authority: self.loan.to_account_info(),

                mint: loan_mint.to_account_info(),
                system_program: self.system_program.to_account_info(),
                token_program: self.token_program.to_account_info(),
            }
        ))?;

        // create collateral_loan_ata
        anchor_spl::associated_token::create(CpiContext::new(
            self.associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: self.lender.to_account_info(),
                associated_token: loan_collateral_ata.to_account_info(),
                authority: self.loan.to_account_info(),

                mint: collateral_mint.to_account_info(),
                system_program: self.system_program.to_account_info(),
                token_program: self.token_program.to_account_info(),
            }
        ))?;

        // transfer loan_mint lender to pda
        token_transfer_user(
            lender_loan_ata.clone(),
            &self.lender, 
            loan_mint_ata.clone(), 
            &self.token_program, 
            loan_amount
        )?;
        Ok(())
    }
}