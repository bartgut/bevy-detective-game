use bevy::prelude::*;
use crate::animation::systems::{
    animation_executor, walking_animation_finished, walking_animation_trigger,
};
use crate::in_game_state::InGameState;

pub mod components;
pub mod systems;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(walking_animation_trigger.run_if(in_state(InGameState::InGame)))
            .add_system(walking_animation_finished.run_if(in_state(InGameState::InGame)))
            .add_system(animation_executor.run_if(in_state(InGameState::InGame)));
    }
}
