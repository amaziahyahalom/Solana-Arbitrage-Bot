use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;
declare_id!("3cmgKnfjYUh7K9o2VWajfiNLTNoFYqtkNkMcRoHptGVJ");

#[program]
pub mod cpi_swap_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::initialize(ctx)
    }

    pub fn raydium_swap(ctx: Context<RaydiumSwap>, amount_in: u64) -> Result<()> {
        instructions::raydium_swap::raydium_swap(ctx, amount_in)
    }

    pub fn whirlpool_swap(ctx: Context<WhirlpoolSwap>, amount_in: u64) -> Result<()> {
        instructions::whirlpool_swap::whirlpool_swap(ctx, amount_in)
    }

    pub fn add_to_whitelist(ctx: Context<ModifyWhitelist>, address: Pubkey) -> Result<()> {
        instructions::add_to_whitelist::add_to_whitelist(ctx, address)
    }

    pub fn remove_from_whitelist(ctx: Context<ModifyWhitelist>, address: Pubkey) -> Result<()> {
        instructions::remove_from_whitelist::remove_from_whitelist(ctx, address)
    }
}