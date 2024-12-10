use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct Object {
    pub value: u32,
    pub bump: u8,
}

impl Object {
    pub const SEED_PREFIX: &'static [u8; 6] = b"object";

    pub fn update(&mut self) {
        self.value = self.value.checked_add(1).unwrap();
    }
}
