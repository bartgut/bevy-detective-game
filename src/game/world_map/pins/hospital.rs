use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::clickable::components::Clickable;
use crate::level_state::LevelState;
use crate::levels::components::LevelTeleport;
use crate::spawnable::components::SpawnableChild;
#[derive(Component)]
pub struct HospitalPin;

impl SpawnableChild for HospitalPin {
    fn spawn_child(&self, parent: &mut EntityCommands, _: &Res<AssetServer>) {
        parent.with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 1.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(50.0, 50.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(-250.0, -100.0, 1.0),
                    ..Default::default()
                },
                HospitalPin,
                LevelTeleport {
                    level_state: LevelState::Morgue,
                },
                Clickable {
                    required_distance: 10000.0,
                },
            ));
        });
    }
}
