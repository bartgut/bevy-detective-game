use bevy::prelude::*;
use crate::game::world_map::pins::city_park::CityParkPin;
use crate::game::world_map::pins::hospital::HospitalPin;
use crate::game::world_map::pins::library::LibraryPin;
use crate::spawnable::components::{Spawnable, SpawnableChild};

#[derive(Component)]
pub struct WorldMap;

#[derive(Component)]
pub struct WorldMapSprite;

impl Spawnable for WorldMap {
    fn spawn(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let mut world_map = commands.spawn((
            SpriteBundle {
                texture: asset_server.load("images/world_map/world_map.png").into(),
                transform: Transform::from_xyz(0.0, 0.0, 2.0).with_scale(Vec3::new(0.7, 0.7, 0.7)),
                ..Default::default()
            },
            WorldMap,
            WorldMapSprite,
        ));
        HospitalPin.spawn_child(&mut world_map, &asset_server);
        LibraryPin.spawn_child(&mut world_map, &asset_server);
        CityParkPin.spawn_child(&mut world_map, &asset_server);
    }
}
