use anchor_lang::prelude::*;

use crate::{
    constant::{TODO_TAG, USER_TAG},
    state::{TodoAccount, UserProfile},
};

pub fn remove_todo(ctx: Context<RemoveTodo>, todo_idx: u8) -> Result<()> {
    // Decreate total todo count
    let user_profile = &mut ctx.accounts.user_profile;
    user_profile.todo_count = user_profile.todo_count.checked_sub(1).unwrap();

    // No need to decrease last todo idx

    // Todo PDA already closed in context

    Ok(())
}

#[derive(Accounts)]
#[instruction(todo_idx: u8)]
pub struct RemoveTodo<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        close = authority,
        seeds = [TODO_TAG, authority.key().as_ref(), &[todo_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
