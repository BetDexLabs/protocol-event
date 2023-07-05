pub mod context;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

use crate::context::*;
use crate::instructions::CreateEventInfo;
use crate::state::event::*;

declare_id!("5qCutonYoeg1aRK31mv4oQYoKdNFMpPaEtDe9nnNQXXf");

#[program]
pub mod protocol_event {
    use super::*;

    pub fn create_event(ctx: Context<CreateEvent>, event_info: CreateEventInfo) -> Result<()> {
        instructions::create(
            &mut ctx.accounts.event,
            event_info,
            ctx.accounts.authority.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.category.key(),
            ctx.accounts.event_group.key(),
        )?;
        Ok(())
    }

    pub fn activate_event(ctx: Context<UpdateEvent>, _slug: String) -> Result<()> {
        instructions::update_event::update_active_flag(&mut ctx.accounts.event, true)
    }

    pub fn deactivate_event(ctx: Context<UpdateEvent>, _slug: String) -> Result<()> {
        instructions::update_event::update_active_flag(&mut ctx.accounts.event, false)
    }

    pub fn update_participants(
        ctx: Context<UpdateEvent>,
        _slug: String,
        participants: Vec<u16>,
    ) -> Result<()> {
        instructions::update_event::update_participants(&mut ctx.accounts.event, participants)
    }

    pub fn update_expected_start_timestamp(
        ctx: Context<UpdateEvent>,
        _slug: String,
        updated_timestamp: i64,
    ) -> Result<()> {
        instructions::update_event::updated_expected_start_timestamp(
            &mut ctx.accounts.event,
            updated_timestamp,
        )
    }

    // Grouping management instructions

    pub fn create_category(ctx: Context<CreateCategory>, code: String, name: String) -> Result<()> {
        instructions::create_grouping::create_category(
            &mut ctx.accounts.category,
            code,
            name,
            ctx.accounts.payer.key(),
        )
    }

    pub fn create_event_group(
        ctx: Context<CreateEventGroup>,
        code: String,
        name: String,
    ) -> Result<()> {
        instructions::create_grouping::create_event_group(
            &mut ctx.accounts.event_group,
            ctx.accounts.category.key(),
            ctx.accounts.payer.key(),
            code,
            name,
        )
    }
}
