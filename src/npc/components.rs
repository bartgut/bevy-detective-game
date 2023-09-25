use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

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
    pub first_dialog_mark: String,
}

impl DialogableNPC {
    pub fn node(&self, is_not_first_dialog: Option<&bool>) -> &str {
        match is_not_first_dialog {
            Some(true) => &self.reset_node,
            _ => &self.start_node,
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
