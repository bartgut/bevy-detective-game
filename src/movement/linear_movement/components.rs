use bevy::prelude::*;

#[derive(Component)]
pub struct Linear2DMovementComponent {
    pub to: Vec2,
    pub velocity: Vec2,
}
