use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::clickable::components::Clickable;
use crate::level_state::LevelState;
use crate::levels::components::LevelTeleport;
use crate::spawnable::components::SpawnableChild;

#[derive(Component)]
pub struct CarInPark;

impl SpawnableChild for CarInPark {
    fn spawn_child(&self, level: &mut EntityCommands, _: &Res<AssetServer>) {
        level.with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(200.0, 200.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(-910.0, -120.0, 1.0)),
                    ..default()
                },
                Clickable {
                    required_distance: 150.0,
                },
                LevelTeleport {
                    level_state: LevelState::LibraryInternals,
                },
            ));
        });
    }
}
