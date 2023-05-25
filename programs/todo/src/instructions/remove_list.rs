use anchor_lang::prelude::*;

use crate::state::{ListAccount, UserProfile};

pub fn remove_list(ctx: Context<RemoveList>, list_idx: u8) -> Result<()> {
    // Decreate total todo count
    let user_profile = &mut ctx.accounts.user_profile;
    user_profile.todo_count = user_profile.todo_count.checked_sub(1).unwrap();

    Ok(())
}

#[derive(Accounts)]
#[instruction(list_idx: u8)]
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
