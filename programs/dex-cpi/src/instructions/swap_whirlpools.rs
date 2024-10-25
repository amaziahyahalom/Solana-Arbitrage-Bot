use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::whitelist::Whitelist;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct SwapWhirlpools<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub whirlpools_program: Program<'info, whirlpools::program::Whirlpools>,
    /// CHECK: This account is checked in the Whirlpools program
    #[account(mut)]
    pub pool_accounts: AccountInfo<'info>,
    pub whitelist: Account<'info, Whitelist>,
}

pub fn swap_whirlpools(
    ctx: Context<SwapWhirlpools>,
    amount_in: u64,
    min_amount_out: u64,
    sqrt_price_limit: u128,
    amount_specified_is_input: bool,
    a_to_b: bool
) -> Result<()> {
    let user = &ctx.accounts.user;
    let whitelist = &ctx.accounts.whitelist;

    require!(whitelist.is_whitelisted(user.key()), ErrorCode::NotWhitelisted);

    let cpi_accounts = whirlpools::cpi::accounts::Swap {
        whirlpool: ctx.accounts.whirlpool.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        token_authority: ctx.accounts.user.to_account_info(),
        token_owner_account_a: ctx.accounts.token_owner_account_a.to_account_info(),
        token_owner_account_b: ctx.accounts.token_owner_account_b.to_account_info(),
        token_vault_a: ctx.accounts.token_vault_a.to_account_info(),
        token_vault_b: ctx.accounts.token_vault_b.to_account_info(),
        tick_array_0: ctx.accounts.tick_array_0.to_account_info(),
        tick_array_1: ctx.accounts.tick_array_1.to_account_info(),
        tick_array_2: ctx.accounts.tick_array_2.to_account_info(),
        oracle: ctx.accounts.oracle.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.whirlpools_program.to_account_info(), cpi_accounts);

    whirlpools::cpi::swap(
        cpi_ctx,
        amount_in,
        min_amount_out,
        sqrt_price_limit,
        amount_specified_is_input,
        a_to_b
    )?;

    Ok(())
}