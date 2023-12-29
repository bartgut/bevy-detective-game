use bevy::prelude::*;
use crate::animation::components::{GridInfo, SpriteAnimationSettings};

#[derive(Component)]
pub struct IdleAnimation;

impl SpriteAnimationSettings for IdleAnimation {
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
            file_path: "images/player/animation/idle.png".to_string(),
            rows: 1,
            columns: 30,
            tile_size: Vec2::new(69.0, 130.0),
        }
    }
}
