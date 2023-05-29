use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct UserProfile {
    pub authority: Pubkey, // 32
    pub name: String,      // 24
    pub username: String,  // 24
    pub password: String,  // 24
}

pub const USER_PROFILE_SPACE: usize = 32 + 24 + 24 + 24; // 104
