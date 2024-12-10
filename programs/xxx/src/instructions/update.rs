use crate::state::Object;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateObject<'info> {
    user: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [
            Object::SEED_PREFIX,
            user.key().as_ref(),
        ],
        bump = object.bump,
    )]
    object: Account<'info, Object>,
}

pub fn update_object(ctx: Context<UpdateObject>) -> Result<()> {
    let object = &mut ctx.accounts.object;
    object.update();
    Ok(())
}
