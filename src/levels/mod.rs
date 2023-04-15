use bevy::prelude::*;
use crate::level_state::LevelState;

pub mod components;
pub mod resource;
pub mod systems;

use systems::*;
use crate::game_state::GameState;

pub struct LevelPlugin;

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub struct LevelCameraInteractions;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(load_current_level.in_schedule(OnEnter(GameState::LevelLoading)))
            .add_system(
                initialize_current_level.in_schedule(OnEnter(GameState::LevelSpriteLoading)),
            )
            .add_system(despawn_current_level.in_schedule(OnExit(GameState::InGame)))
            .add_systems(
                (
                    keyboard_level_input.in_set(LevelCameraInteractions),
                    keyboard_camera_move_blocked_when_border_reached
                        .in_set(LevelCameraInteractions),
                )
                    .in_set(OnUpdate(GameState::InGame)),
            );
    }
}
