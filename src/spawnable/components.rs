use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub trait Spawnable {
    fn spawn(&self, commands: &mut Commands, asset_server: &Res<AssetServer>);
}

pub trait SpawnableChild {
    fn spawn_child(&self, parent: &mut EntityCommands, asset_server: &Res<AssetServer>);
}
