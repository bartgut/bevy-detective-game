use bevy::prelude::*;
use crate::game_state::GameState;
use crate::in_game_state::InGameState;
use crate::player::systems::{cursor_position, despawn_player, initialize_player};

pub mod animation;
pub mod components;
mod constants;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), initialize_player)
            .add_systems(OnExit(GameState::InGame), despawn_player)
            .add_systems(
                Update,
                cursor_position
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(InGameState::InGame)),
            );
    }
}
