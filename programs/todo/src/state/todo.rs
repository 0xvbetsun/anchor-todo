use anchor_lang::prelude::*;

#[account]
pub struct TodoAccount {
    pub authority: Pubkey, // 32
    pub idx: u8,           // 1
    pub content: String,   // ?
    pub marked: bool,      // 1
}
