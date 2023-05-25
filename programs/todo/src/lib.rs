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

    pub fn create_list(ctx: Context<CreateList>, content: String) -> Result<()> {
        instructions::create_list(ctx, content)
    }

    pub fn delete_list(ctx: Context<RemoveList>, idx: u8) -> Result<()> {
        instructions::remove_list(ctx, idx)
    }

    pub fn update_list(ctx: Context<UpdateList>, idx: u8) -> Result<()> {
        instructions::update_List(ctx, idx)
    }
}
