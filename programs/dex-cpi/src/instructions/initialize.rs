use anchor_lang::prelude::*;
use crate::state::whitelist::Whitelist;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 * crate::constants::MAX_WHITELIST_SIZE
    )]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;
    whitelist.authority = ctx.accounts.authority.key();
    whitelist.users = Vec::new();
    Ok(())
}