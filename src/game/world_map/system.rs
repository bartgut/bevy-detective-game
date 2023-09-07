use bevy::prelude::*;
use crate::game::world_map::audio::{CloseMapSoundEffect, OpenMapSoundEffect};
use crate::game::world_map::world_map::{WorldMap};
use crate::spawnable::components::Spawnable;

pub fn show_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(OpenMapSoundEffect);
    WorldMap.spawn(&mut commands, &asset_server);
}

pub fn close_map(mut commands: Commands, map: Query<Entity, With<WorldMap>>) {
    for entity in map.iter() {
        commands.entity(entity).despawn_recursive();
        commands.spawn(CloseMapSoundEffect);
    }
}
