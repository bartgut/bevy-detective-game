use bevy::prelude::*;

pub mod components;
mod constants;
pub mod resource;
pub mod systems;

use systems::*;
use crate::assets::asset_loading_monitor::AssetLoadingStateChangeExt;
use crate::game_state::GameState;
use crate::levels::components::CurrentLevel;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LevelLoading), load_current_level)
            .add_component_loading_state::<CurrentLevel, GameState>(
                GameState::LevelLoading,
                GameState::LevelSpriteLoading,
            )
            .add_systems(
                OnEnter(GameState::LevelSpriteLoading),
                initialize_current_level,
            )
            .add_systems(OnExit(GameState::InGame), despawn_current_level)
            .add_systems(
                Update,
                level_change_trigger_handler.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                on_level_state_change.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    keyboard_level_input,
                    keyboard_camera_move_blocked_when_border_reached,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
