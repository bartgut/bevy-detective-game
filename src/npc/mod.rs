use bevy::prelude::*;
use crate::level_state::LevelState;
use crate::npc::systems::{dialogable_npc_can_start_dialog, initialize_npc, print_when_hovered_over_npc};

pub mod components;
pub mod systems;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(initialize_npc.in_schedule(OnEnter(LevelState::TrainPlatform)))
            .add_system(dialogable_npc_can_start_dialog)
            .add_system(print_when_hovered_over_npc);
    }
}