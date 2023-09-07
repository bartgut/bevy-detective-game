use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::prelude::OnExit;
use crate::game::world_map::system::{close_map, show_map};
use crate::in_game_state::InGameState;

pub mod audio;
mod pins;
pub mod system;
pub mod world_map;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::Map), show_map)
            .add_systems(OnExit(InGameState::Map), close_map);
    }
}
