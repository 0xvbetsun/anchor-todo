use anchor_lang::prelude::*;

use crate::{state::{ListAccount, UserProfile}, constants::LIST_TAG};

pub fn update_list(
    ctx: Context<UpdateList>,
    _id: u8,
    title: String,
    description: String,
) -> Result<()> {
    let list_account = &mut ctx.accounts.list_account;
    list_account.title = title;
    list_account.description = description;

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u8, title: String, description: String)]
pub struct UpdateList<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
    )]
    pub list_account: Box<Account<'info, ListAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
