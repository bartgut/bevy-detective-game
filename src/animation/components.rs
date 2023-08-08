use bevy::prelude::*;

#[derive(Component)]
pub struct IdleSettings {
    pub idle_sprite_texture: String,
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct WalkingSettings {
    pub walking_sprite_texture: String,
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);
