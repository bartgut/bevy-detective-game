use bevy::prelude::*;

#[derive(Component)]
pub struct LevelDescription {
    pub level_name: String,
    pub level_initial_position: Vec3,
    pub player_initial_position: Vec3,
}

#[derive(Component)]
pub struct CurrentLevel;

#[derive(Component)]
pub struct CurrentLevelSprite;

#[derive(Component)]
pub struct TrainPlatformLevel;
