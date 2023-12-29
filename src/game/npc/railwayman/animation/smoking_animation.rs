use bevy::prelude::*;
use crate::animation::components::{GridInfo, SpriteAnimationSettings};

#[derive(Component)]
pub struct SmokingAnimation;

impl SpriteAnimationSettings for SmokingAnimation {
    fn get_first(&self) -> usize {
        0
    }

    fn get_last(&self) -> usize {
        74
    }

    fn animation_pace(&self) -> f32 {
        0.06
    }

    fn get_grid_info(&self) -> GridInfo {
        GridInfo {
            file_path: "images/npc/railwayman/smoking.png".to_string(),
            rows: 1,
            columns: 75,
            tile_size: Vec2::new(68.0, 120.0),
        }
    }
}
