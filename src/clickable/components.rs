use bevy::prelude::*;

#[derive(Component)]
pub struct Clickable {
    pub clickable_texture: String,
    pub level_initial_position: Vec3,
}

#[derive(Component)]
pub struct CanBeClicked;

#[derive(Component)]
pub struct HoveredOverClickable;

#[derive(Component)]
pub struct Clicked;
