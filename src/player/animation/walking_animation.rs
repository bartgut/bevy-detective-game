use bevy::prelude::*;
use crate::animation::components::{GridInfo, SpriteAnimationSettings};

#[derive(Component)]
pub struct WalkingAnimation;

impl SpriteAnimationSettings for WalkingAnimation {
    fn get_first(&self) -> usize {
        0
    }

    fn get_last(&self) -> usize {
        29
    }

    fn animation_pace(&self) -> f32 {
        0.03
    }

    fn get_grid_info(&self) -> GridInfo {
        GridInfo {
            file_path: "images/player/animation/walking.png".to_string(),
            rows: 1,
            columns: 30,
            tile_size: Vec2::new(69.0, 109.0),
        }
    }
}
