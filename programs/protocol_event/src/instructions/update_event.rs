use anchor_lang::prelude::*;

use crate::error::EventError;
use crate::state::event::Event;

pub fn update_active_flag(event: &mut Event, active: bool) -> Result<()> {
    event.active = active;
    Ok(())
}

pub fn add_participants(
    participants: &mut Vec<u16>,
    participants_to_add: Vec<u16>,
    category_participant_count: u16,
) -> Result<()> {
    if participants_to_add.is_empty() {
        return Ok(());
    }

    require!(
        participants_to_add
            .iter()
            .all(|&participant| participant > 0 && participant <= category_participant_count),
        EventError::InvalidEventParticipants,
    );

    participants.extend(participants_to_add.into_iter());
    participants.sort_by(|a, b| a.partial_cmp(b).unwrap());
    participants.dedup();

    require!(
        participants.len() <= Event::MAX_PARTICIPANTS,
        EventError::MaxParticipantsExceeded
    );

    Ok(())
}

pub fn remove_participants(
    participants: &mut Vec<u16>,
    participants_to_remove: Vec<u16>,
) -> Result<()> {
    if participants_to_remove.is_empty() || participants.is_empty() {
        return Ok(());
    }

    participants.retain(|participant| !participants_to_remove.contains(participant));

    Ok(())
}

pub fn updated_expected_start_timestamp(event: &mut Event, timestamp: i64) -> Result<()> {
    event.expected_start_timestamp = timestamp;
    Ok(())
}

#[cfg(test)]
mod tests {
    use anchor_lang::error;

    use crate::error::EventError;
    use crate::instructions::{add_participants, remove_participants};
    use crate::state::event::Event;

    // add participants

    #[test]
    fn test_add_participants() {
        let existing_participants = &mut vec![1, 2, 3, 4];
        let participants_to_add = vec![5, 6, 7, 8];

        add_participants(existing_participants, participants_to_add, 10).unwrap();

        assert_eq!(existing_participants, &vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_add_participants_empty_vec() {
        let existing_participants = &mut vec![1, 2, 3, 4];
        let participants_to_add = vec![];

        add_participants(existing_participants, participants_to_add, 10).unwrap();

        assert_eq!(existing_participants, &vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_add_participants_dedup() {
        let existing_participants = &mut vec![1, 2, 3, 4, 5];
        let participants_to_add = vec![5, 6, 7, 8];

        add_participants(existing_participants, participants_to_add, 10).unwrap();

        assert_eq!(existing_participants, &vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_add_participants_list_full() {
        let existing_participants: &mut Vec<u16> = &mut vec![];
        for i in 0..Event::MAX_PARTICIPANTS {
            existing_participants.push((i + 1) as u16);
        }

        let participants_to_add = vec![301];

        let result = add_participants(existing_participants, participants_to_add, 500);

        assert_eq!(result, Err(error!(EventError::MaxParticipantsExceeded)));
    }

    #[test]
    fn test_add_participants_invalid_participant() {
        let existing_participants = &mut vec![1, 2, 3, 4, 5];
        let participants_to_add = vec![11];

        let result = add_participants(existing_participants, participants_to_add, 10);

        assert_eq!(result, Err(error!(EventError::InvalidEventParticipants)));
    }

    // remove participants

    #[test]
    fn test_remove_participants() {
        let existing_participants = &mut vec![1, 2, 3, 4];
        let participants_to_remove = vec![1, 2];

        remove_participants(existing_participants, participants_to_remove).unwrap();

        assert_eq!(existing_participants, &vec![3, 4]);
    }

    #[test]
    fn test_remove_participants_empty_vecs() {
        let existing_participants = &mut vec![];
        let participants_to_remove = vec![];

        remove_participants(existing_participants, participants_to_remove).unwrap();

        assert_eq!(existing_participants, &vec![]);
    }
}
