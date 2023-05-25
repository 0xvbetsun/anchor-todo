use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile {
    pub authority: Pubkey, // 32
    pub id: u16,           // 2
    pub last_list_idx: u8, // 2
    pub name: String,      // 24
    pub username: String,  // 24
    pub password: String,  // 24
}

impl UserProfile {
    pub fn space(name: &str, username: &str, password: &str) -> usize {
        // discriminator + authority pubkey + id + last_list_idx
        8 + 32 + 2 + 1 +
            // name string
            4 + name.len() +
            // username string
            4 + username.len()  +
            // password string
            4 + password.len()
    }
}
