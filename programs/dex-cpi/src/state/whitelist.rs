use anchor_lang::prelude::*;
use crate::constants::MAX_WHITELIST_SIZE;

#[account]
pub struct Whitelist {
    pub authority: Pubkey,
    pub users: Vec<Pubkey>,
}

impl Whitelist {
    pub fn is_whitelisted(&self, user: &Pubkey) -> bool {
        self.users.contains(user)
    }

    pub fn add_user(&mut self, user: Pubkey) -> Result<()> {
        require!(self.users.len() < MAX_WHITELIST_SIZE, ErrorCode::WhitelistFull);
        if !self.is_whitelisted(&user) {
            self.users.push(user);
        }
        Ok(())
    }

    pub fn remove_user(&mut self, user: Pubkey) -> Result<()> {
        self.users.retain(|&u| u != user);
        Ok(())
    }
}