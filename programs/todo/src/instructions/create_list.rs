use anchor_lang::prelude::*;

use crate::{
    constants::LIST_TAG,
    state::{ListAccount, UserProfile},
};

pub fn create_list(ctx: Context<CreateList>, title: String, description: String) -> Result<()> {
    let list_account = &mut ctx.accounts.list_account;
    let user_profile = &mut ctx.accounts.user_profile;

    list_account.id = user_profile.list_idx;
    list_account.authority = user_profile.key();
    list_account.title = title;
    list_account.description = description;

    user_profile.list_idx += 1;

    Ok(())
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct CreateList<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        payer = authority,
        space = ListAccount::LEN,
        seeds = [LIST_TAG, user_profile.key().as_ref(), &[user_profile.list_idx]],
        bump,
    )]
    pub list_account: Box<Account<'info, ListAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
