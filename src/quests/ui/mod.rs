use bevy::prelude::*;
use crate::in_game_state::InGameState;
use crate::quests::ui::systems::{on_enter, on_exit, quest_buttons, quest_buttons_interaction};

pub mod components;
pub mod systems;

pub struct QuestLogUIPlugin;

impl Plugin for QuestLogUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::QuestLog), on_enter)
            .add_systems(
                Update,
                quest_buttons.run_if(in_state(InGameState::QuestLog)),
            )
            .add_systems(
                Update,
                quest_buttons_interaction.run_if(in_state(InGameState::QuestLog)),
            )
            .add_systems(OnExit(InGameState::QuestLog), on_exit);
    }
}
