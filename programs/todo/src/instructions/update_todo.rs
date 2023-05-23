use anchor_lang::prelude::*;

use crate::{constant::{USER_TAG, TODO_TAG}, state::{UserProfile, TodoAccount}, errors::TodoError};

pub fn update_todo(ctx: Context<UpdateTodo>, todo_idx: u8) -> Result<()> {
    let todo_account = &mut ctx.accounts.todo_account;
    require!(!todo_account.marked, TodoError::AlreadyMarked);

    // Mark todo
    todo_account.marked = true;
    Ok(())
}


#[derive(Accounts)]
#[instruction(todo_idx: u8)]
pub struct UpdateTodo<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [TODO_TAG, authority.key().as_ref(), &[todo_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}