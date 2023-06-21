use anchor_lang::prelude::*;
use crate::state::type_size::{CHAR_SIZE, PUB_KEY_SIZE, U16_SIZE, vec_size};

pub const MAX_CODE_LENGTH: usize = 8;
pub const MAX_NAME_STRING_LENGTH: usize = 50;

#[account]
pub struct Category {
    pub code: String,
    pub name: String,
    pub participant_count: u16,
    pub payer: Pubkey,
}

impl Category {
    pub const SIZE: usize =
        PUB_KEY_SIZE
            + vec_size(CHAR_SIZE, MAX_CODE_LENGTH)
            + vec_size(CHAR_SIZE, MAX_NAME_STRING_LENGTH)
            + U16_SIZE;
}

#[account]
pub struct EventGroup {
    pub category: Pubkey,
    pub code: String,
    pub name: String,
    pub payer: Pubkey,
}

impl EventGroup {
    pub const SIZE: usize =
        PUB_KEY_SIZE * 2
            + vec_size(CHAR_SIZE, MAX_CODE_LENGTH)
            + vec_size(CHAR_SIZE, MAX_NAME_STRING_LENGTH);
}
