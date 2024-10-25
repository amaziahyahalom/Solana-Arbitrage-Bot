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
        instructions::initialize::handler(ctx)
    }

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn swap_raydium(
        ctx: Context<SwapRaydium>,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<()> {
        instructions::swap_raydium::handler(ctx, amount_in, minimum_amount_out)
    }

    pub fn swap_whirlpool(
        ctx: Context<SwapWhirlpool>,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
        amount_specified_is_input: bool,
        a_to_b: bool,
    ) -> Result<()> {
        instructions::swap_whirlpool::handler(
            ctx,
            amount,
            other_amount_threshold,
            sqrt_price_limit,
            amount_specified_is_input,
            a_to_b,
        )
    }

    pub fn manage_whitelist(ctx: Context<ManageWhitelist>, user: Pubkey, add: bool) -> Result<()> {
        instructions::manage_whitelist::handler(ctx, user, add)
    }
}