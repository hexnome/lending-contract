use anchor_lang::prelude::*;

pub use AgioError::*;

#[error_code]
pub enum AgioError {
   
    #[msg("IncorrectConfigAccount")]
    IncorrectConfigAccount,

    #[msg("IncorrectAuthority")]
    IncorrectAuthority,

    #[msg("Incorrect team wallet address")]
    IncorrectTeamWallet,

    #[msg("Loan already lended")]
    AlreadyLended,

    #[msg("Loan already repaid")]
    AlreadyRepaid,

    #[msg("Loan already expried")]
    AlreadyExpired,

    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("Invalid collateral")]
    InvalidCollateral,

    #[msg("Overflow or underflow occured")]
    OverflowOrUnderflowOccurred,

    #[msg("Loan is activated")]
    LoanActivated,
}
