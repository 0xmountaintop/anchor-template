use crate::state::Object;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
pub struct CreateObject<'info> {
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    user: Signer<'info>,

    #[account(
        init,
        space = 8 + Object::INIT_SPACE,
        payer = user,
        seeds = [
            Object::SEED_PREFIX,
            token_mint.key().as_ref(),
            user.key().as_ref(),
        ],
        bump,
    )]
    object: Account<'info, Object>,
    system_program: Program<'info, System>,
}

pub fn create_object(ctx: Context<CreateObject>) -> Result<()> {
    *ctx.accounts.object = Object {
        value: 0,
        token_mint: ctx.accounts.token_mint.key(),
        user: ctx.accounts.user.key(),
    };

    Ok(())
}
