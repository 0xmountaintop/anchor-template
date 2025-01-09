use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct Object {
    pub value: u32,

    pub token_mint: Pubkey,
    pub user: Pubkey,
}

impl Object {
    pub const SEED_PREFIX: &'static [u8; 6] = b"object";

    pub fn set(&mut self, value: u32) {
        self.value = value;
    }
}
