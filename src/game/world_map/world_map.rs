use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::clickable::components::Clickable;
use crate::level_state::LevelState;
use crate::levels::components::LevelTeleport;
use crate::spawnable::components::{Spawnable, SpawnableChild};

#[derive(Component)]
pub struct WorldMap;

#[derive(Component)]
pub struct CityParkPin;

impl SpawnableChild for CityParkPin {
    fn spawn_child(&self, parent: &mut EntityCommands, _: &Res<AssetServer>) {
        parent.with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(50.0, 50.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(-350.0, -150.0, 1.0),
                    ..Default::default()
                },
                CityParkPin,
                LevelTeleport {
                    level_state: LevelState::CityPark,
                },
                Clickable {
                    level_initial_position: Vec3::new(-350.0, -150.0, 1.0),
                    required_distance: 10000.0,
                },
            ));
        });
    }
}

#[derive(Component)]
pub struct WorldMapSprite;

impl Spawnable for WorldMap {
    fn spawn(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("images/map/world_map.png").into(),
                    transform: Transform::from_xyz(0.0, 0.0, 2.0)
                        .with_scale(Vec3::new(0.7, 0.7, 0.7)),
                    ..Default::default()
                },
                WorldMap,
                WorldMapSprite,
            ))
            .id()
    }
}
