use bevy::prelude::{Commands, EventWriter, Query, Res};
use crate::event_journal::components::{ComponentToEvent, JournalEventMessage};
use crate::global_state::global_state::GlobalState;
use crate::quests::components::{Quest, QuestStatus};

pub fn init_quests(mut commands: Commands) {
    commands.spawn(Quest {
        short_description: "Porozmawiaj z kolejarzem".to_string(),
        long_description: "Porozmawiaj z kolejarzem o morderstwie".to_string(),
        status: QuestStatus::Inactive,
        activation_condition: |_| true,
        completion_condition: |context| context.get_value("railwayman_already_talked").filter(|v| **v == true).is_some(),
    });
}

pub fn activate_quests(
    mut query: Query<&mut Quest>,
    mut event_sender: EventWriter<JournalEventMessage>,
    state_context: Res<GlobalState>
) {
    for mut quest in query.iter_mut() {
        match quest.status {
            QuestStatus::Inactive => {
                if (quest.activation_condition)(&*state_context) {
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
    state_context: Res<GlobalState>
) {
    for mut quest in query.iter_mut() {
        match quest.status {
            QuestStatus::Active => {
                if (quest.completion_condition)(&*state_context) {
                    quest.status = QuestStatus::Complete;
                    event_sender.send(quest.to_event());
                }
            }
            _ => (),
        }
    }
}
