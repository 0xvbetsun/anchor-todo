use anchor_lang::prelude::*;
use serde::Deserialize;

#[account]
#[derive(Default, Debug, Deserialize)]
pub struct UserProfile {
    pub authority: Pubkey, // 32
    pub name: String,      // 24
    pub username: String,  // 24
    pub password: String,  // 24
    pub lists: Vec<Pubkey>,
}

impl UserProfile {
    pub fn space(name: &str, username: &str, password: &str) -> usize {
        // discriminator + authority pubkey + id + last_list_idx
        8 + 32 +
            // name string
            4 + name.len() +
            // username string
            4 + username.len()  +
            // password string
            4 + password.len() +
            // vec of item pubkeys
            4 + (u8::MAX as usize) * std::mem::size_of::<Pubkey>()
    }
}
