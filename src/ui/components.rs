use bevy::prelude::{Component, Timer};

#[derive(Component)]
pub struct InvisibleToVisibleTransition(pub Timer);

#[derive(Component)]
pub struct FullScreenText;
