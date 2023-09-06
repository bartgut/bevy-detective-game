use bevy::prelude::*;
use crate::spawnable::components::Spawnable;

#[derive(Component)]
pub struct WorldMap;

#[derive(Component)]
pub struct WorldMapSprite;

impl Spawnable for WorldMap {
    fn spawn(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("images/world_map/world_map.png").into(),
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
