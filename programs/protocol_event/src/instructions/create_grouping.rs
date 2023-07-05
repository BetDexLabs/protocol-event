use crate::state::category::Category;
use crate::state::event_group::EventGroup;
use anchor_lang::prelude::*;

pub fn create_category(
    category: &mut Category,
    code: String,
    name: String,
    payer: Pubkey,
) -> Result<()> {
    category.payer = payer;
    category.code = code;
    category.name = name;
    category.participant_count = 0;

    Ok(())
}

pub fn create_event_group(
    event_group: &mut EventGroup,
    category: Pubkey,
    payer: Pubkey,
    code: String,
    name: String,
) -> Result<()> {
    event_group.category = category;
    event_group.payer = payer;
    event_group.code = code;
    event_group.name = name;

    Ok(())
}
