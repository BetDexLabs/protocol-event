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

    // Event management instructions

    pub fn create_event(ctx: Context<CreateEvent>, event_info: CreateEventInfo) -> Result<()> {
        instructions::create_event::create(ctx, event_info)?;
        Ok(())
    }

    pub fn activate_event(ctx: Context<UpdateEvent>, _slug: String) -> Result<()> {
        instructions::update_event::update_active_flag(&mut ctx.accounts.event, true)
    }

    pub fn deactivate_event(ctx: Context<UpdateEvent>, _slug: String) -> Result<()> {
        instructions::update_event::update_active_flag(&mut ctx.accounts.event, false)
    }

    pub fn add_event_participants(
        ctx: Context<UpdateEvent>,
        _slug: String,
        participants_to_add: Vec<u16>,
    ) -> Result<()> {
        instructions::update_event::add_participants(
            &mut ctx.accounts.event.participants,
            participants_to_add,
        )
    }

    pub fn remove_event_participants(
        ctx: Context<UpdateEvent>,
        _slug: String,
        participants_to_remove: Vec<u16>,
    ) -> Result<()> {
        instructions::update_event::remove_participants(
            &mut ctx.accounts.event.participants,
            participants_to_remove,
        )
    }

    pub fn update_event_expected_start_timestamp(
        ctx: Context<UpdateEvent>,
        _slug: String,
        updated_timestamp: i64,
    ) -> Result<()> {
        instructions::update_event::update_expected_start_timestamp(
            &mut ctx.accounts.event,
            updated_timestamp,
        )
    }

    pub fn update_event_actual_start_timestamp(
        ctx: Context<UpdateEvent>,
        _slug: String,
        updated_timestamp: i64,
    ) -> Result<()> {
        instructions::update_event::update_actual_start_timestamp(
            &mut ctx.accounts.event,
            updated_timestamp,
        )
    }

    pub fn update_event_actual_end_timestamp(
        ctx: Context<UpdateEvent>,
        _slug: String,
        updated_timestamp: i64,
    ) -> Result<()> {
        instructions::update_event::update_actual_end_timestamp(
            &mut ctx.accounts.event,
            updated_timestamp,
        )
    }

    pub fn update_event_name(
        ctx: Context<UpdateEvent>,
        _slug: String,
        updated_name: String,
    ) -> Result<()> {
        instructions::update_event::update_name(&mut ctx.accounts.event, updated_name)
    }

    // Participant management instructions

    pub fn create_individual_participant(
        ctx: Context<CreateParticipant>,
        code: String,
        name: String,
    ) -> Result<()> {
        instructions::create_participant::create_individual_participant(ctx, code, name)
    }

    pub fn create_team_participant(
        ctx: Context<CreateParticipant>,
        code: String,
        name: String,
    ) -> Result<()> {
        instructions::create_participant::create_team_participant(ctx, code, name)
    }

    // Grouping management instructions

    pub fn create_category(ctx: Context<CreateCategory>, code: String, name: String) -> Result<()> {
        instructions::create_grouping::create_category(ctx, code, name)
    }

    pub fn create_event_group(
        ctx: Context<CreateEventGroup>,
        code: String,
        name: String,
    ) -> Result<()> {
        instructions::create_grouping::create_event_group(ctx, code, name)
    }
}
