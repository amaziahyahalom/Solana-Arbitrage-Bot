use anchor_lang::prelude::*;
use crate::state::whitelist::Whitelist;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct ManageWhitelist<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, has_one = authority @ ErrorCode::Unauthorized)]
    pub whitelist: Account<'info, Whitelist>,
}

pub fn handler(ctx: Context<ManageWhitelist>, user: Pubkey, add: bool) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;

    if add {
        whitelist.add_user(user)?;
    } else {
        whitelist.remove_user(user)?;
    }
    Ok(())
}