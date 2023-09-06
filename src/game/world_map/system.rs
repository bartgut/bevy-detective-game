use bevy::prelude::*;
use crate::game::world_map::pins::city_park::CityParkPin;
use crate::game::world_map::pins::hospital::HospitalPin;
use crate::game::world_map::pins::library::LibraryPin;
use crate::game::world_map::world_map::{WorldMap};
use crate::in_game_state::InGameState;
use crate::spawnable::components::{Spawnable, SpawnableChild};

pub fn show_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Query<Entity, With<WorldMap>>,
    keyboard_buttons: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<InGameState>>,
) {
    if keyboard_buttons.just_pressed(KeyCode::M) {
        match map.get_single() {
            Ok(entity) => {
                commands.entity(entity).despawn_recursive();
                game_state.set(InGameState::InGame);
            }
            Err(_) => {
                game_state.set(InGameState::Map);
                let map_id = WorldMap.spawn(&mut commands, &asset_server);
                CityParkPin.spawn_child(&mut commands.entity(map_id), &asset_server);
                HospitalPin.spawn_child(&mut commands.entity(map_id), &asset_server);
                LibraryPin.spawn_child(&mut commands.entity(map_id), &asset_server);
            }
        }
    }
}

pub fn delete_map(mut commands: Commands, map: Query<Entity, With<WorldMap>>) {
    for entity in map.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
