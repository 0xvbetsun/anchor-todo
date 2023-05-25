use anchor_lang::prelude::*;

use super::Status;

#[account]
pub struct ListAccount {
    pub authority: Pubkey,   // 32
    pub title: String,       // 24
    pub description: String, // 24
    pub status: Status,      // 24
    pub todos: Vec<Pubkey>,
}

impl ListAccount {
    fn space(title: &str, description: &str) -> usize {
        // discriminator + authority pubkey + capacity
        8 + 32 + 1 +
            // title string
            4 + title.len() +
            // description string
            4 + description.len()  +
            // status
            24 +
            // vec of item pubkeys
            4 + (u8::MAX as usize) * std::mem::size_of::<Pubkey>()
    }
}
