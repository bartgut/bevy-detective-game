use bevy::asset::AssetServer;
use bevy::prelude::{Assets, Commands, EventReader, EventWriter, Res, ResMut};
use crate::dialogs::dialog_runner::context::StateContext;
use crate::event_journal::components::JournalEventMessage;
use crate::global_state::global_state::GlobalState;
use crate::quests::components::{QuestStatus, QuestSystem};
use crate::quests::loader::format::{QuestBundle, QuestEventMessage};

pub fn init_quests(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(QuestSystem {
        quest: asset_server.load("quests/chapter1.quest"),
    });
}

pub fn activate_quest_v2(
    quest_system: Res<QuestSystem>,
    mut quest_asset: ResMut<Assets<QuestBundle>>,
    state_context: Res<GlobalState>,
    mut event_writer: EventWriter<QuestEventMessage>,
) {
    if let Some(quest_bundle) = quest_asset.get_mut(&quest_system.quest) {
        for quest in quest_bundle.quests.iter_mut() {
            let should_be_activated = quest.activation_condition.iter().fold(true, |acc, flag| {
                acc && state_context.get_value(flag).unwrap_or(&false).clone()
            });

            if should_be_activated && quest.status == QuestStatus::Inactive {
                quest.status = QuestStatus::Active;
                event_writer.send(QuestEventMessage::NewQuest(quest.id.clone()));
            }
        }
    }
}

pub fn to_journal_event(
    mut quest_events_reader: EventReader<QuestEventMessage>,
    mut journal_event_writer: EventWriter<JournalEventMessage>,
    quest_system: Res<QuestSystem>,
    quest_asset: Res<Assets<QuestBundle>>,
) {
    for event in quest_events_reader.read() {
        if let Some(quest_bundle) = quest_asset.get(&quest_system.quest) {
            match event {
                QuestEventMessage::NewQuest(id) => {
                    if let Some(quest) = quest_bundle.quests.iter().find(|q| q.id == *id) {
                        journal_event_writer.send(JournalEventMessage::NewQuest(format!(
                            "Nowe zadanie: {}",
                            quest.name.clone()
                        )));
                    }
                }
                QuestEventMessage::QuestCompleted(id) => {
                    if let Some(quest) = quest_bundle.quests.iter().find(|q| q.id == *id) {
                        journal_event_writer.send(JournalEventMessage::QuestCompleted(format!(
                            "Zadanie wykonane: {}",
                            quest.name.clone()
                        )));
                    }
                }
            }
        }
    }
}

pub fn complete_quest_v2(
    quest_system: Res<QuestSystem>,
    mut quest_asset: ResMut<Assets<QuestBundle>>,
    state_context: Res<GlobalState>,
    mut event_writer: EventWriter<QuestEventMessage>,
) {
    if let Some(quest_bundle) = quest_asset.get_mut(&quest_system.quest) {
        for quest in quest_bundle.quests.iter_mut() {
            let should_be_activated = quest.completion_condition.iter().fold(true, |acc, flag| {
                acc && state_context.get_value(flag).unwrap_or(&false).clone()
            });

            if should_be_activated && quest.status == QuestStatus::Active {
                quest.status = QuestStatus::Complete;
                event_writer.send(QuestEventMessage::QuestCompleted(quest.id.clone()));
            }
        }
    }
}
