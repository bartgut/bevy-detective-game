use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;
use crate::game_state::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), initialize_main_menu_ui)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu_ui)
            .add_systems(
                Update,
                (main_menu_interaction).run_if(in_state(GameState::MainMenu)),
            );
    }
}
