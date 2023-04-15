use bevy::prelude::*;
use crate::level_state::LevelState;

#[derive(Component)]
pub struct NPC {
    pub texture_file: String,
    pub level_initial_position: Vec3,
}

#[derive(Component)]
pub struct DialogableNPC {
    pub dialog_file_name: String,
    pub start_node: String,
    pub reset_node: String,
}

#[derive(Component)]
pub struct CanStartDialog;

#[derive(Component)]
pub struct HoveredOverNPC;
