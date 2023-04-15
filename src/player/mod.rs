use bevy::prelude::*;
use crate::game_state::GameState;
use crate::in_game_state::InGameState;
use crate::level_state::LevelState;
use crate::player::systems::{cursor_position, despawn_player, initialize_player};

pub mod components;
mod constants;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(initialize_player.in_schedule(OnEnter(GameState::InGame)))
            .add_system(despawn_player.in_schedule(OnExit(GameState::InGame)))
            .add_system(
                cursor_position
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(InGameState::InGame)),
            );
    }
}
