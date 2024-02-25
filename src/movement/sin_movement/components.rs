use bevy::prelude::*;

#[derive(Component)]
pub struct SinMovementComponent {
    pub velocity: Vec2,
    pub to: Vec2,
    pub amplitude: f32,
    pub frequency: f32,
}
