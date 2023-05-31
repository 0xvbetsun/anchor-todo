use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct ListAccount {
    pub authority: Pubkey,  
    pub id: u8,             
    pub title: String,      
    pub description: String,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TITLE_LENGTH: usize = 20;
const MAX_DESCRIPTION_LENGTH: usize = 20;
const INDEX_LENGTH: usize = 1;

impl ListAccount {
    pub const LEN: usize = DISCRIMINATOR_LENGTH 
    + PUBLIC_KEY_LENGTH  // authority
    + INDEX_LENGTH // index
    + STRING_LENGTH_PREFIX + MAX_TITLE_LENGTH // title
    + STRING_LENGTH_PREFIX + MAX_DESCRIPTION_LENGTH; // description
}