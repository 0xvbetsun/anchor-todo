use anchor_lang::prelude::*;
use instructions::*;

declare_id!("5kZtVwH69P8uUH6fZ1Dd4Fh55H4254vNnigWZ8VAZirp");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

#[program]
pub mod todo {
    use super::*;

    pub fn initialize(ctx: Context<InitializeUser>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn create_todo(ctx: Context<CreateTodo>, content: String) -> Result<()> {
        instructions::create_todo(ctx, content)
    }

    pub fn delete_todo(ctx: Context<RemoveTodo>, idx: u8) -> Result<()> {
        instructions::remove_todo(ctx, idx)
    }

    pub fn update_todo(ctx: Context<UpdateTodo>, idx: u8) -> Result<()> {
        instructions::update_todo(ctx, idx)
    }
}

pub fn is_zero_account(account_info: &AccountInfo) -> bool {
    let account_data: &[u8] = &account_info.data.borrow();
    let len = account_data.len();
    let mut is_zero = true;
    for i in 0..len - 1 {
        if account_data[i] != 0 {
            is_zero = false;
        }
    }
    is_zero
}

pub fn bump(seeds: &[&[u8]], program_id: &Pubkey) -> u8 {
    let (_found_key, bump) = Pubkey::find_program_address(seeds, program_id);
    bump
}