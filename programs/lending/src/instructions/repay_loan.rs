use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token}};

use crate::{constants::{CONFIG, LOAN}, errors::AgioError, state::{Config, Loan}, utils::{token_transfer_user, token_transfer_with_signer}};



#[derive(Accounts)]
pub struct RepayLoan<'info> {

    #[account(
        mut,
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,

    #[account(
        mut,
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
            borrower.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            collateral_mint.key().as_ref(),
        ],
        seeds::program = anchor_spl::associated_token::ID,
        bump,
    )]
    borrower_collateral_ata: AccountInfo<'info>,

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
            team_wallet.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            loan_mint.key().as_ref(),
        ],
        bump,
        seeds::program = anchor_spl::associated_token::ID
    )]
    team_loan_ata: AccountInfo<'info>,

    #[account(
        mut,
        constraint = team_wallet.key() == global_config.team_wallet @AgioError::IncorrectAuthority
    )]
    pub team_wallet: AccountInfo<'info>,

    #[account(mut)]
    pub borrower: Signer<'info>,

    pub loan_key: Account<'info, Mint>,
    pub lender: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,

}

impl<'info> RepayLoan<'info> {
    pub fn process(
        &mut self,
        loan_bump: u8
    ) -> Result<()> {

        let global_config = &mut self.global_config;

        let loan = &mut self.loan;
        let loan_collateral_ata = &mut self.loan_collateral_ata;
        let borrower_collateral_ata = &mut self.borrower_collateral_ata;
        let lender_loan_ata = &mut self.lender_loan_ata;
        let borrower_loan_ata = &mut self.borrower_loan_ata;
        let team_loan_ata = &mut self.team_loan_ata;

        require!(loan.repaid == false, AgioError::AlreadyRepaid);
        loan.repaid = true;

        // create borrower collateral_mint ata if it doesn't exit
        if borrower_collateral_ata.data_is_empty() {
            anchor_spl::associated_token::create(CpiContext::new(
                self.associated_token_program.to_account_info(),
                anchor_spl::associated_token::Create {
                    payer: self.borrower.to_account_info(),
                    associated_token: borrower_collateral_ata.to_account_info(),
                    authority: self.borrower.to_account_info(),

                    mint: self.collateral_mint.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                }
            ))?;
        }

        // create team_wallet loan_mint ata if it doesn't exit
        if lender_loan_ata.data_is_empty() {
            anchor_spl::associated_token::create(CpiContext::new(
                self.associated_token_program.to_account_info(),
                anchor_spl::associated_token::Create {
                    payer: self.borrower.to_account_info(),
                    associated_token: lender_loan_ata.to_account_info(),
                    authority: self.borrower.to_account_info(),

                    mint: self.loan_mint.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                }
            ))?;
        }

        // create lender loan_mint ata if it doesn't exit
        if team_loan_ata.data_is_empty() {
            anchor_spl::associated_token::create(CpiContext::new(
                self.associated_token_program.to_account_info(),
                anchor_spl::associated_token::Create {
                    payer: self.borrower.to_account_info(),
                    associated_token: team_loan_ata.to_account_info(),
                    authority: self.borrower.to_account_info(),

                    mint: self.loan_mint.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                }
            ))?;
        }

        let fee_amount = (loan.loan_amount / 100)
            .checked_mul(global_config.lend_fee)
            .ok_or(AgioError::OverflowOrUnderflowOccurred)?;

        // trnasfer collateral_mint pda to borrower
        token_transfer_with_signer(
            loan_collateral_ata.clone(), 
            loan.to_account_info(), 
            borrower_collateral_ata.clone(), 
            &self.token_program, 
            &[&[LOAN.as_bytes(), loan.key().as_ref(), &[loan_bump]]], 
            loan.collateral_amount
        )?;

        // transfer loan_mint borrower to lender
        token_transfer_user(
            borrower_loan_ata.clone(), 
            &self.borrower, 
            lender_loan_ata.clone(), 
            &self.token_program, 
            loan.loan_amount - fee_amount
        )?;

        // transfer loan_mint borrower to team_wallet
        token_transfer_user(
            borrower_loan_ata.clone(), 
            &self.borrower, 
            team_loan_ata.clone(),
            &self.token_program, 
            fee_amount * 2
        )?;
        
        Ok(())
    }
}