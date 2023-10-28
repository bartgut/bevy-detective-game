use bevy::prelude::*;

#[derive(Component)]
pub struct JournalEventUI;

#[derive(Event)]
pub enum JournalEventMessage {
    AddedToInventory(String),
    NewQuest(String),
    QuestCompleted(String),
}

pub trait ComponentToEvent {
    fn to_event(&self) -> JournalEventMessage;
}
