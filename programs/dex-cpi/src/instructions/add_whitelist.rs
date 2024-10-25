use anchor_lang::prelude::*;
use crate::state::whitelist::Whitelist;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct ModifyWhitelist<'info> {
    #[account(mut)]
    pub whitelist: Account<'info, Whitelist>,
    pub authority: Signer<'info>,
}

pub fn add_whitelist(ctx: Context<ModifyWhitelist>, address: Pubkey) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;
    require!(whitelist.authority == ctx.accounts.authority.key(), ErrorCode::Unauthorized);

    if !whitelist.addresses.contains(&address) {
        whitelist.addresses.push(address);
    }
    Ok(())
}