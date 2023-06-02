use anchor_lang::prelude::*;

use crate::state::{ListAccount, UserProfile};

pub fn remove_list(_ctx: Context<RemoveList>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct RemoveList<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        close = authority,
    )]
    pub list_account: Box<Account<'info, ListAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
