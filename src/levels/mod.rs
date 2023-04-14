use bevy::prelude::*;
use crate::level_state::LevelState;

pub mod components;
pub mod systems;

use systems::*;
use crate::game_state::GameState;

pub struct TrainPlatformLevelPlugin;

impl Plugin for TrainPlatformLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(initialize_train_platform_level.in_schedule(OnEnter(GameState::InGame)))
            .add_system(despawn_train_platform_level.in_schedule(OnExit(GameState::InGame)))
            .add_system(keyboard_level_input)
            .add_system(keyboard_camera_move_blocked_when_border_reached);
    }
}
