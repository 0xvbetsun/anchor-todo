use anchor_lang::prelude::*;

use crate::state::{ListAccount, UserProfile};

pub fn remove_list(ctx: Context<RemoveList>) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    let list_account = &ctx.accounts.list_account;
    // remove list
    if let Some(idx) = user_profile
        .lists
        .iter()
        .position(|k| *k == list_account.key())
    {
        user_profile.lists.remove(idx);
    }
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
        has_one = authority,
    )]
    pub list_account: Box<Account<'info, ListAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
