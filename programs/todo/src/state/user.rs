use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct UserProfile {
    pub authority: Pubkey,
    pub name: String,
    pub username: String,
    pub password: String,
    pub list_idx: u8,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_NAME_LENGTH: usize = 20;
const MAX_USERNAME_LENGTH: usize = 20;
const MAX_PASSWORD_LENGTH: usize = 20;
const LIST_INDEX_LENGTH: usize = 1;

impl UserProfile {
    pub const LEN: usize = DISCRIMINATOR_LENGTH 
    + PUBLIC_KEY_LENGTH  // authority
    + STRING_LENGTH_PREFIX + MAX_NAME_LENGTH // name
    + STRING_LENGTH_PREFIX + MAX_USERNAME_LENGTH // username
    + STRING_LENGTH_PREFIX + MAX_PASSWORD_LENGTH // password
    + LIST_INDEX_LENGTH; // list index
}
