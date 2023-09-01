use bevy::prelude::*;

#[derive(Component)]
pub struct Clickable {
    pub level_initial_position: Vec3,
    pub required_distance: f32,
}

#[derive(Component)]
pub struct CanBeClicked;

#[derive(Component)]
pub struct HoveredOverClickable;

#[derive(Component)]
pub struct Clicked;
