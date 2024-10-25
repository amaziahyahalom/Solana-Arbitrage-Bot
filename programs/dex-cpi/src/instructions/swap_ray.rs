use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::state::whitelist::Whitelist;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct SwapRaydium<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub user_source_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_destination_token: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub pool_program_id: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub amm_id: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub pool_coin_token_account: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub pool_pc_token_account: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub serum_program_id: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub serum_bids: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub serum_asks: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub serum_event_queue: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub serum_coin_vault_account: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub serum_pc_vault_account: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub serum_vault_signer: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub raydium_program: Program<'info, raydium::program::Raydium>,
    /// CHECK: This account is checked in the Raydium program
    #[account(mut)]
    pub pool_accounts: AccountInfo<'info>,
    pub whitelist: Account<'info, Whitelist>,
}

pub fn handler(ctx: Context<SwapRaydium>, amount_in: u64, min_amount_out: u64) -> Result<()> {
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

// This function should construct the instruction data for Raydium swap
// You'll need to implement this based on Raydium's instruction format
fn raydium_swap_instruction(amount_in: u64, min_amount_out: u64) -> Vec<u8> {
    let mut instruction_data = Vec::new();
    instruction_data.extend_from_slice(&(0u8).to_le_bytes()); // Instruction discriminator for swap
    instruction_data.extend_from_slice(&amount_in.to_le_bytes());
    instruction_data.extend_from_slice(&min_amount_out.to_le_bytes());
    // Add any other necessary parameters
    instruction_data
}