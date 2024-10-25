use anchor_lang::prelude::*;
use crate::state::whitelist::Whitelist;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct SetPause<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, has_one = authority @ ErrorCode::Unauthorized)]
    pub whitelist: Account<'info, Whitelist>,
}

pub fn set_pause(ctx: Context<SetPause>, paused: bool) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;
    whitelist.set_paused(paused);
    Ok(())
}