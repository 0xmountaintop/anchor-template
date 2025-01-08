use crate::state::Object;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetObject<'info> {
    user: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [
            Object::SEED_PREFIX,
            user.key().as_ref(),
        ],
        bump,
    )]
    object: Account<'info, Object>,
}

pub fn set_object(ctx: Context<SetObject>, value: u32) -> Result<()> {
    let object = &mut ctx.accounts.object;
    object.set(value);
    Ok(())
}
