use bevy::app::{App, Plugin, Update};
use bevy::prelude::OnEnter;
use crate::event_journal::components::JournalEventMessage;
use crate::event_journal::system::{on_event_received, ui_setup};
use crate::game_state::GameState;

pub mod components;
pub mod system;

pub struct EventJournalPlugin;

impl Plugin for EventJournalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<JournalEventMessage>()
            .add_systems(OnEnter(GameState::Intro), ui_setup)
            .add_systems(Update, on_event_received);
    }
}
