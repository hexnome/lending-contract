use anchor_lang::prelude::*;

use crate::{constants::LOAN, errors::AgioError, state::Loan, utils::{token_transfer_user, token_transfer_with_signer}};
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

#[derive(Accounts)]
pub struct BorrowLoan<'info> {

    #[account(
        mut,
        seeds = [LOAN.as_bytes(), &lender.key().to_bytes(), &loan_key.key().to_bytes()],
        bump,
    )]
    pub loan: Account<'info, Loan>,


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
            borrower.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            loan_mint.key().as_ref(),
        ],
        bump,
        seeds::program = anchor_spl::associated_token::ID
    )]
    borrower_loan_ata: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            borrower.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            collateral_mint.key().as_ref(),
        ],
        bump,
        seeds::program = anchor_spl::associated_token::ID
    )]
    borrower_collateral_ata: AccountInfo<'info>,

    pub lender: AccountInfo<'info>,

    pub collateral_mint: Account<'info, Mint>,

    pub loan_key: Account<'info, Mint>,
    #[account(mut)]
    pub borrower: Signer<'info>,
   
    #[account(mut)]
    pub program_collateral_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> BorrowLoan<'info>{
    pub fn process(
        &mut self,
        collateral_amount: u64,
        loan_bump: u8,
    ) -> Result<()> {

        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        let loan = &mut self.loan;
        let loan_collateral_ata = &mut self.loan_collateral_ata;
        let loan_mint_ata = &mut self.loan_mint_ata;
        let borrower_loan_ata = &mut self.borrower_loan_ata;
        let borrower_collateral_ata = &mut self.borrower_collateral_ata;
        let borrower = &self.borrower;
        
        require!(loan.borrower == Pubkey::default(), AgioError::AlreadyLended);
        require!(loan.expire_date > current_time, AgioError::AlreadyExpired);

        //  create user wallet ata, if it doean't exit
        if borrower_loan_ata.data_is_empty() {
            anchor_spl::associated_token::create(CpiContext::new(
                self.associated_token_program.to_account_info(),
                anchor_spl::associated_token::Create {
                    payer: self.borrower.to_account_info(),
                    associated_token: borrower_loan_ata.to_account_info(),
                    authority: self.borrower.to_account_info(),

                    mint: self.loan_mint.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                }
            ))?;
        }

        
        loan.borrower = self.borrower.key();
        loan.borrow_date = current_time;
        
        loan.collateral_amount = collateral_amount;
        
        // loan_mint transfer from laon to borrower
        token_transfer_with_signer(
            loan_mint_ata.clone(), 
            loan.to_account_info(), 
            borrower_loan_ata.clone(), 
            &self.token_program,
            &[&[LOAN.as_bytes(), loan.key().as_ref(), &[loan_bump]]], 
            loan.loan_amount
        )?;

        // collateral_mint transfer from borrower to loan
        token_transfer_user(
            borrower_collateral_ata.clone(), 
            borrower, 
            loan_collateral_ata.clone(), 
            &self.token_program, 
            collateral_amount
        )?;

        Ok(())
    }
}