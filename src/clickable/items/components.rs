use bevy::prelude::*;
use crate::global_state::global_state::AddCollectibleToInventory;

#[derive(Component, Clone)]
pub struct Collectible {
    pub inventory_sprite: String,
    pub name: String,
    pub description: String,
}

impl Collectible {
    pub fn on_collect(&self, commands: &mut Commands) {
        commands.spawn(AddCollectibleToInventory(self.clone()));
    }
}
