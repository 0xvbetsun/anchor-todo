use anchor_lang::prelude::*;

use crate::state::{ListAccount, UserProfile, Status};

pub fn create_list(ctx: Context<CreateList>, _title: String, _description: String) -> Result<()> {
    let list_account = &mut ctx.accounts.list_account;
    let user_profile = &mut ctx.accounts.user_profile;

    // Fill contents with argument
    list_account.authority = ctx.accounts.authority.key();
    list_account.idx = user_profile.last_list_idx;
    list_account.title = _title;
    list_account.description = _description;
    list_account.status = Status::Active;

    // Increase list idx for PDA
    user_profile.last_list_idx = user_profile.last_list_idx.checked_add(1).unwrap();

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct CreateList<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        payer = authority,
        space = std::mem::size_of::<ListAccount>() + 8,
    )]
    pub list_account: Box<Account<'info, ListAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
