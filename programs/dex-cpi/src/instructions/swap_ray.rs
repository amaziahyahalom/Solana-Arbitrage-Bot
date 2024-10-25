use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::whitelist::Whitelist;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct SwapRaydium<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub raydium_program: Program<'info, raydium::program::Raydium>,
    /// CHECK: This account is checked in the Raydium program
    #[account(mut)]
    pub pool_accounts: AccountInfo<'info>,
    pub whitelist: Account<'info, Whitelist>,
}

pub fn swap_ray(ctx: Context<SwapRaydium>, amount_in: u64, min_amount_out: u64) -> Result<()> {
    let user = &ctx.accounts.user;
    let whitelist = &ctx.accounts.whitelist;

    require!(whitelist.is_whitelisted(user.key()), ErrorCode::NotWhitelisted);

    let cpi_accounts = raydium::cpi::accounts::Swap {
        token_program: ctx.accounts.token_program.to_account_info(),
        user: ctx.accounts.user.to_account_info(),
        source: ctx.accounts.source.to_account_info(),
        destination: ctx.accounts.destination.to_account_info(),
        pool_accounts: ctx.accounts.pool_accounts.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.raydium_program.to_account_info(), cpi_accounts);

    raydium::cpi::swap(cpi_ctx, amount_in, min_amount_out)?;
    Ok(())
}