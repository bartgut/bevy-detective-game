use bevy::prelude::*;
use crate::movement::linear_movement::systems::linear_2d_movement_executor;

pub mod linear_movement;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, linear_2d_movement_executor);
    }
}
