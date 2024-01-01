use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_yarnspinner::dialog_runner::context::StateContext;
use crate::clickable::components::{Clickable};
use crate::conditional_visibility::components::VisibilityCondition;
use crate::global_state::global_state::ConditionFunc;
use crate::level_state::LevelState;
use crate::levels::components::LevelTeleport;
use crate::spawnable::components::SpawnableChild;

#[derive(Component)]
pub struct CityParkPin;

impl SpawnableChild for CityParkPin {
    fn spawn_child(&self, parent: &mut EntityCommands, _: &Res<AssetServer>) {
        parent.with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 1.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(50.0, 50.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(200.0, -50.0, 1.0),
                    ..Default::default()
                },
                CityParkPin,
                VisibilityCondition {
                    conditions: vec![ConditionFunc::StateCondition(|state| {
                        *state.get_value("lecturer_location_known").unwrap_or(&false) == true
                    })],
                },
                LevelTeleport {
                    level_state: LevelState::CityPark,
                },
                Clickable {
                    required_distance: 10000.0,
                },
            ));
        });
    }
}
