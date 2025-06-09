use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token}};

use crate::{constants::{CONFIG, LOAN}, errors::AgioError, state::{Config, Loan}, utils::token_transfer_with_signer};

#[derive(Accounts)]
pub struct CancelLoan<'info> {

    #[account(
        mut,
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,
    
    #[account(
        mut,
        
        seeds = [LOAN.as_bytes(), &lender.key().to_bytes(), &loan_key.key().to_bytes()],
        bump,
    )]
    loan: Account<'info, Loan>,

    
    pub collateral_mint: Account<'info, Mint>,
    
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
            team_wallet.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            collateral_mint.key().as_ref(),
        ],
        bump,
        seeds::program = anchor_spl::associated_token::ID
    )]
    team_collateral_ata: AccountInfo<'info>,
        
    #[account(
        mut,
        seeds = [
            lender.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            collateral_mint.key().as_ref(),
            ],
        bump,
        seeds::program = anchor_spl::associated_token::ID
    )]
    lender_collateral_ata: AccountInfo<'info>,
    
    #[account(
        mut,
        constraint = team_wallet.key() == global_config.team_wallet @AgioError::IncorrectAuthority,
    )]
    pub team_wallet: AccountInfo<'info>,
    
    #[account(mut)]
    pub lender: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub loan_key: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

}

impl<'info> CancelLoan<'info> {
    pub fn process(
        &mut self,
        loan_bump: u8
    ) -> Result<()> {
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        let global_config = &mut self.global_config;
        let loan = &mut self.loan;
        let lender_collateral_ata = &mut self.lender_collateral_ata;
        let loan_collateral_ata = &mut self.loan_collateral_ata;
        let team_collateral_ata = &mut self.team_collateral_ata;

        let expire_loan = loan.borrow_date
            .checked_add(loan.duration)
            .ok_or(AgioError::OverflowOrUnderflowOccurred)?;

        require!(loan.lender == self.lender.key(), AgioError::IncorrectAuthority);
        require!(
            loan.borrower != Pubkey::default() || expire_loan < current_time, 
            AgioError::LoanActivated
        );
        
        // create lender collateral_mint ata if it doesn't exit
        if lender_collateral_ata.data_is_empty() {
            anchor_spl::associated_token::create(CpiContext::new(
                self.associated_token_program.to_account_info(),
                anchor_spl::associated_token::Create {
                    payer: self.lender.to_account_info(),
                    associated_token: lender_collateral_ata.to_account_info(),
                    authority: self.lender.to_account_info(),

                    mint: self.collateral_mint.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                }
            ))?;
        }

        // create team_wallet collateral_mint ata if it doesn't exit
        if team_collateral_ata.data_is_empty() {
            anchor_spl::associated_token::create(CpiContext::new(
                self.associated_token_program.to_account_info(),
                anchor_spl::associated_token::Create {
                    payer: self.lender.to_account_info(),
                    associated_token: team_collateral_ata.to_account_info(),
                    authority: self.lender.to_account_info(),

                    mint: self.collateral_mint.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                }
            ))?;
        }


        // trnasfer collateral_mint to lender and teamWallet
        let fee_amount = (loan.collateral_amount / 100)
            .checked_mul(global_config.lend_fee)
            .ok_or(AgioError::OverflowOrUnderflowOccurred)?;


        token_transfer_with_signer(
            loan_collateral_ata.clone(),
            loan.to_account_info(), 
            lender_collateral_ata.clone(), 
            &self.token_program, 
            &[&[LOAN.as_bytes(), loan.key().as_ref(), &[loan_bump]]], 
            loan.collateral_amount - fee_amount
        )?;

        token_transfer_with_signer(
            loan_collateral_ata.clone(),
            self.team_wallet.to_account_info(), 
            self.team_collateral_ata.clone(), 
            &self.token_program, 
            &[&[LOAN.as_bytes(), loan.key().as_ref(), &[loan_bump]]], 
            fee_amount
        )?;

        
        Ok(())
    }
}