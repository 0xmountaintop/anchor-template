#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("your_program_id_here");

#[program]
pub mod xxx {
    use super::*;

    pub fn create_object(ctx: Context<CreateObject>) -> Result<()> {
        create::create_object(ctx)
    }

    pub fn set_object(ctx: Context<SetObject>, value: u32) -> Result<()> {
        set::set_object(ctx, value)
    }
}
