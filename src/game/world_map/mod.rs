use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::prelude::OnExit;
use crate::game::world_map::system::{delete_map, show_map};
use crate::in_game_state::InGameState;

mod pins;
pub mod system;
pub mod world_map;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_map)
            .add_system(delete_map.in_schedule(OnExit(InGameState::Map)));
    }
}
