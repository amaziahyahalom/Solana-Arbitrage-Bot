use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;
pub mod constants;

use instructions::*;

declare_id!("YourProgramIDHere");

#[program]
pub mod cpi_swap_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::initialize(ctx)
    }

    pub fn swap_ray(ctx: Context<SwapRaydium>, amount_in: u64, min_amount_out: u64) -> Result<()> {
        instructions::swap_ray::swap_ray(ctx, amount_in, min_amount_out)
    }

    pub fn swap_whirlpools(ctx: Context<SwapWhirlpools>, amount_in: u64, min_amount_out: u64) -> Result<()> {
        instructions::swap_whirlpools::swap_whirlpools(ctx, amount_in, min_amount_out)
    }

    pub fn manage_whitelist(ctx: Context<ManageWhitelist>, user: Pubkey, add: bool) -> Result<()> {
        instructions::manage_whitelist::manage_whitelist(ctx, user, add)
    }
}