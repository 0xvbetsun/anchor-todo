use anchor_lang::prelude::*;

use crate::{
    constant::{TODO_TAG, USER_TAG},
    state::{TodoAccount, UserProfile},
};

pub fn create_todo(ctx: Context<CreateTodo>, _content: String) -> Result<()> {
    let todo_account = &mut ctx.accounts.todo_account;
    let user_profile = &mut ctx.accounts.user_profile;

    // Fill contents with argument
    todo_account.authority = ctx.accounts.authority.key();
    todo_account.idx = user_profile.last_todo;
    todo_account.content = _content;
    todo_account.marked = false;

    // Increase todo idx for PDA
    user_profile.last_todo = user_profile.last_todo.checked_add(1).unwrap();

    // Increase total todo count
    user_profile.todo_count = user_profile.todo_count.checked_add(1).unwrap();

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct CreateTodo<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [TODO_TAG, authority.key().as_ref(), &[user_profile.last_todo as u8].as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<TodoAccount>() + 8,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
