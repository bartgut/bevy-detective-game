use bevy::prelude::*;
use crate::event_journal::components::{ComponentToEvent, JournalEventMessage};

pub enum QuestStatus {
    Inactive,
    Active,
    Complete,
}

#[derive(Component)]
pub struct Quest {
    pub short_description: String,
    pub long_description: String,
    pub status: QuestStatus,
    pub activation_condition: fn() -> bool,
    pub completion_condition: fn() -> bool,
}

impl ComponentToEvent for Quest {
    fn to_event(&self) -> JournalEventMessage {
        match self.status {
            QuestStatus::Active => JournalEventMessage {
                message: format!("Nowe zadanie: {}", self.long_description.clone()),
            },
            QuestStatus::Complete => JournalEventMessage {
                message: format!("You completed the quest: {}", self.short_description),
            },
            _ => JournalEventMessage {
                message: "".to_string(),
            },
        }
    }
}
