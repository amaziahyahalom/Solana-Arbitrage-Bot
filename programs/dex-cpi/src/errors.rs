use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("User is not whitelisted")]
    NotWhitelisted,
    #[msg("Unauthorized to modify whitelist")]
    Unauthorized,
}