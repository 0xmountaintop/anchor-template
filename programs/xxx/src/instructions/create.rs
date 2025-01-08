use crate::state::Object;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateObject<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        init,
        space = 8 + Object::INIT_SPACE,
        payer = payer,
        seeds = [
            Object::SEED_PREFIX,
            payer.key().as_ref(),
        ],
        bump,
    )]
    object: Account<'info, Object>,
    system_program: Program<'info, System>,
}

pub fn create_object(ctx: Context<CreateObject>) -> Result<()> {
    *ctx.accounts.object = Object {
        value: 0,
    };

    Ok(())
}
