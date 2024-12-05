// File: src/lib.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use anchor_lang::solana_program::instruction::Instruction;

declare_id!("CRQXfRGq3wTkjt7JkqhojPLiKLYLjHPGLebnfiiQB46T");

pub mod error;
pub mod state;
pub mod ix_data;
pub mod swaps;

use error::ErrorCode;
use state::SwapState;
use ix_data::SwapData;
pub use swaps::*;

#[program]
pub mod arbitrage_bot {
    use super::*;

    pub fn init_program(ctx: Context<InitSwapState>, input_token: Pubkey) -> Result<()> {
        let swap_state = &mut ctx.accounts.swap_state;
        swap_state.swap_input = 0;
        swap_state.is_valid = false;  
        swap_state.input_token = input_token;
        swap_state.current_token = input_token;
        Ok(())
    }

    pub fn start_swap(ctx: Context<TokenAndSwapState>, swap_input: u64) -> Result<()> {
        let swap_state = &mut ctx.accounts.swap_state;
        swap_state.start_balance = ctx.accounts.src.amount;
        swap_state.swap_input = swap_input;
        swap_state.is_valid = true;
        Ok(())
    }

    pub fn execute_arbitrage(ctx: Context<ExecuteArbitrage>, path: Vec<ArbitrageStep>) -> Result<()> {
        let swap_state = &mut ctx.accounts.swap_state;
        
        // Execute each step in the arbitrage path
        for step in path {
            match step {
                ArbitrageStep::Orca(amount_in, minimum_amount_out) => {
                    orca_swap(ctx.accounts.orca.clone(), amount_in, minimum_amount_out)?;
                },
                ArbitrageStep::Raydium(amount_in, minimum_amount_out) => {
                    raydium_swap(ctx.accounts.raydium.clone(), amount_in, minimum_amount_out)?;
                },
                ArbitrageStep::Meteora(amount_in, minimum_amount_out) => {
                    meteora_swap(ctx.accounts.meteora.clone(), amount_in, minimum_amount_out)?;
                },
                ArbitrageStep::Phoenix(amount_in, minimum_amount_out) => {
                    phoenix_swap(ctx.accounts.phoenix.clone(), amount_in, minimum_amount_out)?;
                },
                ArbitrageStep::Lifinity(amount_in, minimum_amount_out) => {
                    lifinity_swap(ctx.accounts.lifinity.clone(), amount_in, minimum_amount_out)?;
                },
                ArbitrageStep::Jupiter(amount_in, minimum_amount_out) => {
                    jupiter_swap(ctx.accounts.jupiter.clone(), amount_in, minimum_amount_out)?;
                },
            }
            
            swap_state.current_token = step.get_output_token();
        }
        
        Ok(())
    }

    pub fn profit_or_revert(ctx: Context<TokenAndSwapState>, path: Vec<ArbitrageStep>) -> Result<()> {
        let swap_state = &mut ctx.accounts.swap_state;
        swap_state.is_valid = false;
    
        let init_balance = swap_state.start_balance;
        let final_balance = ctx.accounts.src.amount;
        
        let total_fees: u64 = path.iter()
            .map(|step| calculate_fees(step, init_balance))
            .sum();
        
        msg!(
            "old = {:?}; new = {:?}; diff = {:?}; fees = {:?}", 
            init_balance, 
            final_balance, 
            final_balance.saturating_sub(init_balance), 
            total_fees
        );
        
        require!(
            final_balance > init_balance.saturating_add(total_fees), 
            ErrorCode::NoProfit
        );
    
        Ok(())
    }

    // DEX-specific swap implementations
    pub fn orca_swap<'info>(
        ctx: Context<'_, '_, '_, 'info, OrcaSwap<'info>>,
        amount_in: u64,
        minimum_amount_out: u64
    ) -> Result<()> {
        let amount_in = prepare_swap(&ctx.accounts.swap_state)?;
        _orca_swap(&ctx, amount_in, minimum_amount_out)?;
        end_swap(&mut ctx.accounts.swap_state, &mut ctx.accounts.user_dst)?;
        Ok(())
    }

    pub fn raydium_swap<'info>(
        ctx: Context<'_, '_, '_, 'info, RaydiumSwap<'info>>,
        amount_in: u64,
        minimum_amount_out: u64
    ) -> Result<()> {
        let amount_in = prepare_swap(&ctx.accounts.swap_state)?;
        _raydium_swap(&ctx, amount_in, minimum_amount_out)?;
        end_swap(&mut ctx.accounts.swap_state, &mut ctx.accounts.user_destination_token_account)?;
        Ok(())
    }

    pub fn meteora_swap<'info>(
        ctx: Context<'_, '_, '_, 'info, MeteoraSwap<'info>>,
        amount_in: u64,
        minimum_amount_out: u64
    ) -> Result<()> {
        let amount_in = prepare_swap(&ctx.accounts.swap_state)?;
        _meteora_swap(&ctx, amount_in, minimum_amount_out)?;
        end_swap(&mut ctx.accounts.swap_state, &mut ctx.accounts.user_output_token_account)?;
        Ok(())
    }

    pub fn jupiter_swap<'info>(
        ctx: Context<'_, '_, '_, 'info, JupiterSwap<'info>>,
        amount_in: u64,
        minimum_amount_out: u64
    ) -> Result<()> {
        let amount_in = prepare_swap(&ctx.accounts.swap_state)?;
        _jupiter_swap(&ctx, amount_in, minimum_amount_out)?;
        end_swap(&mut ctx.accounts.swap_state, &mut ctx.accounts.user_destination_token)?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitSwapState<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + SwapState::LEN,
        seeds = [b"swap_state"],
        bump
    )]
    pub swap_state: Account<'info, SwapState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TokenAndSwapState<'info> {
    pub src: Account<'info, TokenAccount>,
    #[account(mut, seeds=[b"swap_state"], bump)]
    pub swap_state: Account<'info, SwapState>,
}

#[derive(Accounts)]
pub struct ExecuteArbitrage<'info> {
    #[account(mut)]
    pub swap_state: Account<'info, SwapState>,
    pub orca: OrcaSwap<'info>,
    pub raydium: RaydiumSwap<'info>,
    pub meteora: MeteoraSwap<'info>,
    pub jupiter: JupiterSwap<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ArbitrageStep {
    Orca(u64, u64),
    Raydium(u64, u64),
    Meteora(u64, u64),
    Jupiter(u64, u64),
}

impl ArbitrageStep {
    fn get_output_token(&self) -> Pubkey {
        // Implementation would return the output token for each DEX step
        // This is a placeholder that should be replaced with actual token addresses
        Pubkey::default()
    }
}

// Helper functions
pub fn prepare_swap(swap_state: &Account<SwapState>) -> Result<u64> {
    require!(swap_state.is_valid, ErrorCode::InvalidState);
    let amount_in = swap_state.swap_input;
    msg!("swap amount in: {:?} for token: {:?}", amount_in, swap_state.current_token);
    Ok(amount_in)
}

pub fn end_swap(
    swap_state: &mut Account<SwapState>,
    user_dst: &mut Account<TokenAccount>
) -> Result<()> {
    let dst_start_balance = user_dst.amount;
    user_dst.reload()?;
    let dst_end_balance = user_dst.amount;
    let swap_amount_out = dst_end_balance.saturating_sub(dst_start_balance);
    msg!(
        "swap amount out: {:?} for token: {:?}", 
        swap_amount_out, 
        swap_state.current_token
    );
    swap_state.swap_input = swap_amount_out;
    Ok(())
}

pub fn calculate_fees(step: &ArbitrageStep, amount: u64) -> u64 {
    match step {
        ArbitrageStep::Orca(_, _) => amount * 30 / 10000,  // 0.3% fee
        ArbitrageStep::Raydium(_, _) => amount * 25 / 10000, // 0.25% fee
        ArbitrageStep::Meteora(_, _) => amount * 20 / 10000, // 0.2% fee
        ArbitrageStep::Jupiter(_, _) => amount * 10 / 10000, // 0.1% fee
    }
}