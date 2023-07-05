use anchor_lang::prelude::*;

use crate::error::EventError;
use crate::state::category::Category;
use crate::state::event::Event;

pub fn increment_category_participant_count(category: &mut Category) -> Result<()> {
    require!(
        (category.participant_count as usize) < Event::MAX_PARTICIPANTS,
        EventError::MaxParticipantsExceeded,
    );

    category.participant_count = category.participant_count.checked_add(1).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::error::EventError;
    use crate::instructions::increment_category_participant_count;
    use crate::state::category::Category;
    use anchor_lang::error;

    #[test]
    fn test_increment_category_participant_count() {
        let mut category = Category {
            code: "".to_string(),
            name: "".to_string(),
            participant_count: 0,
            payer: Default::default(),
        };

        let result = increment_category_participant_count(&mut category);
        assert!(result.is_ok());
        assert_eq!(category.participant_count, 1);
    }

    #[test]
    fn test_increment_category_participant_count_limit_exceeded() {
        let mut category = Category {
            code: "".to_string(),
            name: "".to_string(),
            participant_count: 300,
            payer: Default::default(),
        };

        let result = increment_category_participant_count(&mut category);
        assert_eq!(result, Err(error!(EventError::MaxParticipantsExceeded)));
    }
}
