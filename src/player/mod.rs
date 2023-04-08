use bevy::prelude::*;
use crate::level_state::LevelState;
use crate::player::systems::{cursor_position, despawn_player, initialize_player};

pub mod systems;
pub mod components;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(initialize_player.in_schedule(OnEnter(LevelState::TrainPlatform)))
            .add_system(despawn_player.in_schedule(OnExit(LevelState::TrainPlatform)))
            .add_system(cursor_position.run_if(in_state(LevelState::TrainPlatform)));
    }
}