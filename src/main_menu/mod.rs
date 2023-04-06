use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;
use crate::game_state::GameState;

pub struct MainMenuPlugin;

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub struct MainMenuInteractions;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(initialize_main_menu_ui.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(despawn_main_menu_ui.in_schedule(OnExit(GameState::MainMenu)))
            .add_systems(
                (
                    new_game_button_click.in_set(MainMenuInteractions),
                    quit_game_button_interaction.in_set(MainMenuInteractions)
                ).in_set(OnUpdate(GameState::MainMenu))
            );
    }
}