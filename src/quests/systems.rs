use bevy::prelude::{Commands, EventWriter, Query};
use crate::event_journal::components::{ComponentToEvent, JournalEventMessage};
use crate::quests::components::{Quest, QuestStatus};

pub fn init_quests(mut commands: Commands) {
    commands.spawn(Quest {
        short_description: "Porozmawiaj z kolejarzem".to_string(),
        long_description: "Porozmawiaj z kolejarzem o morderstwie".to_string(),
        status: QuestStatus::Inactive,
        activation_condition: || true,
        completion_condition: || false,
    });
}

pub fn activate_quests(
    mut query: Query<&mut Quest>,
    mut event_sender: EventWriter<JournalEventMessage>,
) {
    for mut quest in query.iter_mut() {
        match quest.status {
            QuestStatus::Inactive => {
                if (quest.activation_condition)() {
                    quest.status = QuestStatus::Active;
                    event_sender.send(quest.to_event());
                }
            }
            _ => (),
        }
    }
}

pub fn complete_quests(
    mut query: Query<&mut Quest>,
    mut event_sender: EventWriter<JournalEventMessage>,
) {
    for mut quest in query.iter_mut() {
        match quest.status {
            QuestStatus::Active => {
                if (quest.completion_condition)() {
                    quest.status = QuestStatus::Complete;
                    event_sender.send(quest.to_event());
                }
            }
            _ => (),
        }
    }
}
