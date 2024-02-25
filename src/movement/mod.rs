use bevy::prelude::*;
use crate::in_game_state::InGameState;
use crate::movement::linear_movement::systems::{linear_2d_movement_executor, linear_2d_movement_stop};
use crate::movement::sin_movement::systems::sin_movement_executor;

pub mod linear_movement;
pub mod sin_movement;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, linear_2d_movement_executor)
            .add_systems(Update, sin_movement_executor)
            .add_systems(OnExit(InGameState::InGame), linear_2d_movement_stop);
    }
}
