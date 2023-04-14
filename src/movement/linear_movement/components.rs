use bevy::prelude::*;

#[derive(Component)]
pub struct LinearMovementComponent {
    pub to: Vec3,
    pub velocity: Vec3,
}
