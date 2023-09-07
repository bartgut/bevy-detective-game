use bevy::prelude::*;
use crate::game_state::GameState;
use crate::npc::systems::{
    dialogable_npc_can_start_dialog, initialize_npcs, print_when_hovered_over_npc,
};

pub mod components;
pub mod resource;
pub mod systems;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), initialize_npcs)
            .add_systems(Update, dialogable_npc_can_start_dialog)
            .add_systems(Update, print_when_hovered_over_npc);
    }
}
