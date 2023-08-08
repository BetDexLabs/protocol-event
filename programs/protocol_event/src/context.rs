use crate::error::EventError;
use crate::instructions::CreateEventInfo;
use crate::state::classification::Classification;
use crate::state::event_group::EventGroup;
use crate::state::participant::Participant;
use crate::state::subcategory::Subcategory;
use crate::Event;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

#[derive(Accounts)]
#[instruction(event_info: CreateEventInfo)]
pub struct CreateEvent<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [
            b"event".as_ref(),
            event_info.code.as_ref()
        ],
        bump,
        space = Event::SIZE
    )]
    pub event: Account<'info, Event>,

    #[account(has_one = subcategory)]
    pub event_group: Account<'info, EventGroup>,
    pub subcategory: Account<'info, Subcategory>,

    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_code: String)]
pub struct UpdateEvent<'info> {
    #[account(
        mut,
        seeds = [
            b"event".as_ref(),
            _code.as_ref()
        ],
        bump,
        has_one = authority,
        has_one = subcategory,
    )]
    pub event: Account<'info, Event>,
    pub subcategory: Account<'info, Subcategory>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(code: String)]
pub struct CreateClassification<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [
            b"classification".as_ref(),
            code.as_ref(),
        ],
        bump,
        space = Classification::SIZE
    )]
    pub classification: Account<'info, Classification>,

    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateClassification<'info> {
    #[account(mut, has_one = authority)]
    pub classification: Account<'info, Classification>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(code: String)]
pub struct CreateSubcategory<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [
            b"subcategory".as_ref(),
            classification.key().as_ref(),
            code.as_ref(),
        ],
        bump,
        space = Subcategory::SIZE
    )]
    pub subcategory: Account<'info, Subcategory>,
    pub classification: Account<'info, Classification>,

    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateSubcategory<'info> {
    #[account(mut, has_one = authority)]
    pub subcategory: Account<'info, Subcategory>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(code: String)]
pub struct CreateEventGroup<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [
            b"event_group".as_ref(),
            subcategory.key().as_ref(),
            code.as_ref(),
        ],
        bump,
        space = EventGroup::SIZE
    )]
    pub event_group: Account<'info, EventGroup>,
    pub subcategory: Account<'info, Subcategory>,

    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateEventGroup<'info> {
    #[account(mut, has_one = authority)]
    pub event_group: Account<'info, EventGroup>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CreateParticipant<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [
            b"participant".as_ref(),
            subcategory.key().as_ref(),
            subcategory.participant_count.to_string().as_ref()
        ],
        bump,
        space = Participant::SIZE
    )]
    pub participant: Account<'info, Participant>,
    #[account(
        mut,
        has_one = authority @ EventError::AuthorityMismatch,
    )]
    pub subcategory: Account<'info, Subcategory>,

    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateParticipant<'info> {
    #[account(mut, has_one = authority)]
    pub participant: Account<'info, Participant>,
    pub authority: Signer<'info>,
}

// close accounts

#[derive(Accounts)]
pub struct CloseEvent<'info> {
    #[account(
        mut,
        has_one = authority,
        has_one = payer,
        close = payer,
    )]
    pub event: Account<'info, Event>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct CloseSubcategory<'info> {
    #[account(
        mut,
        has_one = authority,
        has_one = payer,
        close = payer,
    )]
    pub subcategory: Account<'info, Subcategory>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct CloseEventGroup<'info> {
    #[account(
        mut,
        has_one = authority,
        has_one = payer,
        close = payer,
    )]
    pub event_group: Account<'info, EventGroup>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct CloseParticipant<'info> {
    #[account(
        mut,
        has_one = authority,
        has_one = payer,
        close = payer,
    )]
    pub participant: Account<'info, Participant>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct CloseClassification<'info> {
    #[account(
        mut,
        has_one = authority,
        has_one = payer,
        close = payer,
    )]
    pub classification: Account<'info, Classification>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: SystemAccount<'info>,
}
