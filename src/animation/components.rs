use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationEnabled;

#[derive(Component)]
pub struct GridInfo {
    pub file_path: String,
    pub rows: usize,
    pub columns: usize,
    pub tile_size: Vec2,
    pub tile_scale: Vec3,
}

impl Default for GridInfo {
    fn default() -> Self {
        Self {
            file_path: "".to_string(),
            rows: 1,
            columns: 1,
            tile_size: Vec2::new(64.0, 64.0),
            tile_scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

pub trait SpriteAnimationSettings {
    fn get_first(&self) -> usize;
    fn get_last(&self) -> usize;
    fn animation_pace(&self) -> f32;
    fn get_grid_info(&self) -> GridInfo;
}
