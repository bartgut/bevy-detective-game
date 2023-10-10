use bevy::prelude::*;

#[derive(Component)]
pub struct JournalEventUI;

#[derive(Event)]
pub struct JournalEventMessage {
    pub message: String,
}

pub trait ComponentToEvent {
    fn to_event(&self) -> JournalEventMessage;
}
