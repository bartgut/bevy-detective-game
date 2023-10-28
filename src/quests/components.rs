use bevy::prelude::*;
use crate::dialogs::dialog_runner::context::StateContext;
use crate::event_journal::components::{ComponentToEvent, JournalEventMessage};
use crate::quests::components::JournalEventMessage::NewQuest;
use crate::quests::components::JournalEventMessage::QuestCompleted;

#[derive(PartialEq)]
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
    pub activation_condition: fn(&dyn StateContext) -> bool,
    pub completion_condition: fn(&dyn StateContext) -> bool,
}

impl ComponentToEvent for Quest {
    fn to_event(&self) -> JournalEventMessage {
        match self.status {
            QuestStatus::Active => NewQuest {
                0: format!("Nowe zadanie: {}", self.long_description),
            },
            QuestStatus::Complete => QuestCompleted {
                0: format!("Zadanie wykonane: {}", self.short_description),
            },
            _ => NewQuest { // TODO change
                0: "".to_string(),
            },
        }
    }
}
