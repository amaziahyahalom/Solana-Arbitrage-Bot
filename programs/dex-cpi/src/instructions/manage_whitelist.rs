use anchor_lang::prelude::*;
use crate::state::whitelist::Whitelist;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct ManageWhitelist<'info> {
    #[account(mut)]
    pub whitelist: Account<'info, Whitelist>,
    #[account(
        constraint = authority.key() == whitelist.authority @ ErrorCode::UnauthorizedAccess
    )]
    pub authority: Signer<'info>,
}

pub fn manage_whitelist(ctx: Context<ManageWhitelist>, user: Pubkey, add: bool) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;

    if add {
        if whitelist.users.contains(&user) {
            return Err(ErrorCode::UserAlreadyWhitelisted.into());
        }
        whitelist.users.push(user);
    } else {
        whitelist.users.retain(|&x| x != user);
    }

    Ok(())
}