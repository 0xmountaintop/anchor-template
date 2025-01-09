use crate::state::Object;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
pub struct SetObject<'info> {
    #[account(
        constraint = token_account.mint == object.token_mint,
        constraint = token_account.mint == token_mint.key(),
        constraint = token_account.owner == object.user,
        constraint = token_account.owner == user.key()
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    user: Signer<'info>,
    #[account(
        mut,
        seeds = [
            Object::SEED_PREFIX,
            token_mint.key().as_ref(),
            user.key().as_ref(),
        ],
        bump,
        constraint = object.user == user.key()
    )]
    object: Account<'info, Object>,
}

pub fn set_object(ctx: Context<SetObject>, value: u32) -> Result<()> {
    let object = &mut ctx.accounts.object;
    object.set(value);
    Ok(())
}
