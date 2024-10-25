use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::Instruction,
    program::invoke,
};
use anchor_spl::token::{Token, TokenAccount};
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
    /// CHECK: This is the Raydium program ID
    pub raydium_program: AccountInfo<'info>,
    /// CHECK: These are the pool accounts required by Raydium
    #[account(mut)]
    pub pool_accounts: AccountInfo<'info>,
    pub whitelist: Account<'info, Whitelist>,
}

pub fn swap_ray(ctx: Context<SwapRaydium>, amount_in: u64, min_amount_out: u64) -> Result<()> {
    let user = &ctx.accounts.user;
    let whitelist = &ctx.accounts.whitelist;

    require!(whitelist.is_whitelisted(user.key()), ErrorCode::NotWhitelisted);

    // Construct the instruction for Raydium swap
    let ix = Instruction {
        program_id: ctx.accounts.raydium_program.key(),
        accounts: vec![
            AccountMeta::new(ctx.accounts.user.key(), true),
            AccountMeta::new(ctx.accounts.source.key(), false),
            AccountMeta::new(ctx.accounts.destination.key(), false),
            AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
            AccountMeta::new(ctx.accounts.pool_accounts.key(), false),
            // Add other necessary account metas for Raydium swap
        ],
        data: raydium_swap_instruction(amount_in, min_amount_out),
    };

    // Invoke the Raydium swap instruction
    invoke(
        &ix,
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.source.to_account_info(),
            ctx.accounts.destination.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.pool_accounts.to_account_info(),
            // Add other necessary account infos
        ],
    )?;

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