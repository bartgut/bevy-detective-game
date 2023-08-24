use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub trait Spawnable {
    fn spawn(&self, level: &mut EntityCommands, asset_server: &Res<AssetServer>);
}
