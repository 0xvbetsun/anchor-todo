use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct ListAccount {
    pub authority: Pubkey,   // 32
    pub title: String,       // 24
    pub description: String, // 24
}


pub const LIST_ACCOUNT_SPACE: usize = 32 + 24 + 24; // 80
