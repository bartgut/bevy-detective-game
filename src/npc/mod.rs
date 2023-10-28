use bevy::prelude::*;
use crate::game_state::GameState;
use crate::npc::systems::{initialize_npcs};

pub mod components;
pub mod resource;
pub mod systems;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InLevelSpritesLoading), initialize_npcs);
    }
}
