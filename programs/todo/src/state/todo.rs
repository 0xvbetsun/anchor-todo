use anchor_lang::prelude::*;

use super::Status;

#[account]
pub struct TodoAccount {
    pub authority: Pubkey,   // 32
    pub list_idx: u8,        // 1
    pub idx: u8,             // 1
    pub title: String,       // 24
    pub description: String, // 24
    pub done: bool,          // 1
    pub status: Status,      // 24
}
