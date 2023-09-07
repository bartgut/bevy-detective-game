use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::clickable::components::Clickable;
use crate::level_state::LevelState;
use crate::levels::components::LevelTeleport;
use crate::spawnable::components::SpawnableChild;
#[derive(Component)]
pub struct LibraryPin;

impl SpawnableChild for LibraryPin {
    fn spawn_child(&self, parent: &mut EntityCommands, _: &Res<AssetServer>) {
        parent.with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 1.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(50.0, 50.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(275.0, -125.0, 1.0),
                    ..Default::default()
                },
                LibraryPin,
                LevelTeleport {
                    level_state: LevelState::TrainPlatform,
                },
                Clickable {
                    level_initial_position: Vec3::new(275.0, -125.0, 1.0),
                    required_distance: 10000.0,
                },
            ));
        });
    }
}