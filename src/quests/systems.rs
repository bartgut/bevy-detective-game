use bevy::prelude::{Commands, EventWriter, Query, Res};
use crate::event_journal::components::{ComponentToEvent, JournalEventMessage};
use crate::global_state::global_state::GlobalState;
use crate::quests::components::{Quest, QuestStatus};

pub fn init_quests(mut commands: Commands) {
    commands.spawn(Quest {
        short_description: "Porozmawiaj z kolejarzem".to_string(),
        long_description: "Porozmawiaj z kolejarzem o morderstwie. To on jako pierwszy znalazl ofiare i poinformowal o tym policje".to_string(),
        status: QuestStatus::Inactive,
        activation_condition: |_| true,
        completion_condition: |context| context.get_value("railwayman_already_talked").filter(|v| **v == true).is_some(),
    });
    commands.spawn(Quest {
        short_description: "Zbadaj plakat wykladu".into(),
        long_description: "Zbadaj plakat wykladu. Moze znajdziesz tam jakies wskazowki".into(),
        status: QuestStatus::Inactive,
        activation_condition: |_| true,
        completion_condition: |context| {
            context
                .get_value("lecturer_name_known")
                .and_then(|name| {
                    context
                        .get_value("lecture_date_known")
                        .map(|date| *name && *date)
                })
                .unwrap_or(false)
        },
    });
}

pub fn activate_quests(
    mut query: Query<&mut Quest>,
    mut event_sender: EventWriter<JournalEventMessage>,
    state_context: Res<GlobalState>,
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
    state_context: Res<GlobalState>,
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
