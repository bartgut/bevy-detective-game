use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::spawnable::components::Spawnable;

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
    pub first_dialog: bool,
}

impl DialogableNPC {
    pub fn node(&self) -> &str {
        if self.first_dialog {
            &self.start_node
        } else {
            &self.reset_node
        }
    }
}

#[derive(Component)]
pub struct CanStartDialog;

#[derive(Component)]
pub struct HoveredOverNPC;

#[derive(Component)]
pub struct NPCInDialog;

pub trait SpawnableNPC {
    fn spawn(&self, level: &mut EntityCommands, asset_server: &Res<AssetServer>);
}
