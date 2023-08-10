use bevy::prelude::*;
use crate::animation::components::{GridInfo, SpriteAnimationSettings};

#[derive(Component)]
pub struct WalkingAnimation;

impl SpriteAnimationSettings for WalkingAnimation {
    fn get_first(&self) -> usize {
        0
    }

    fn get_last(&self) -> usize {
        25
    }

    fn get_grid_info(&self) -> GridInfo {
        GridInfo {
            file_path: "images/player/animation/walking.png".to_string(),
            rows: 1,
            columns: 26,
            tile_size: Vec2::new(64.0, 114.0),
            tile_scale: Vec3::new(1.5, 1.5, 1.0),
        }
    }
}
