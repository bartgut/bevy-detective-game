use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::animation::components::{AnimationEnabled, GridInfo, SpriteAnimationSettings};
use crate::spawnable::components::SpawnableChild;

#[derive(Component)]
pub struct TrainSmoke;

#[derive(Component)]
pub struct TrainSmokeAnimation;

impl SpriteAnimationSettings for TrainSmokeAnimation {
    fn get_first(&self) -> usize {
        0
    }

    fn get_last(&self) -> usize {
        149
    }

    fn animation_pace(&self) -> f32 {
        0.06
    }

    fn get_grid_info(&self) -> GridInfo {
        GridInfo {
            file_path: "images/particles/trainsmoke.png".to_string(),
            rows: 1,
            columns: 150,
            tile_size: Vec2::new(84.0, 150.0),
            tile_scale: Vec3::new(2.0, 2.0, 2.0),
        }
    }
}

impl SpawnableChild for TrainSmoke {
    fn spawn_child(&self, level: &mut EntityCommands, _: &Res<AssetServer>) {
        level.with_children(|parent| {
            parent.spawn((
                TrainSmoke,
                AnimationEnabled,
                TrainSmokeAnimation,
                Transform::from_translation(Vec3::new(300.0, 220.0, 1.0)),
            ));
        });
    }
}
