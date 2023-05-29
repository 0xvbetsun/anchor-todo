use anchor_lang::prelude::*;

use crate::state::{ListAccount, UserProfile};

pub fn create_list(ctx: Context<CreateList>, title: String, description: String) -> Result<()> {
    let list_account = &mut ctx.accounts.list_account;
    let user_profile = &mut ctx.accounts.user_profile;

    // Fill contents with argument
    list_account.authority = ctx.accounts.authority.key();
    list_account.title = title;
    list_account.description = description;

    // add list to user profile
    // user_profile.lists.push(list_account.key());

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
        space = ListAccount::space(&title, &description),
    )]
    pub list_account: Box<Account<'info, ListAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
