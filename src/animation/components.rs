use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Component)]
pub struct AnimationEnabled;

#[derive(Component)]
pub struct GridInfo {
    pub file_path: String,
    pub rows: usize,
    pub columns: usize,
    pub tile_size: Vec2,
}

impl Default for GridInfo {
    fn default() -> Self {
        Self {
            file_path: "".to_string(),
            rows: 1,
            columns: 1,
            tile_size: Vec2::new(64.0, 64.0),
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

#[derive(Resource, Default)]
pub struct AnimationHandles {
    pub handles: HashMap<String, Handle<TextureAtlas>>,
}
