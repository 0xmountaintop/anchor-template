#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("your_program_id_here");

#[program]
pub mod anchor_program_example {
    use super::*;

    pub fn create_object(ctx: Context<CreateObject>) -> Result<()> {
        create::create_object(ctx)
    }

    pub fn update_object(ctx: Context<UpdateObject>) -> Result<()> {
        update::update_object(ctx)
    }
}
