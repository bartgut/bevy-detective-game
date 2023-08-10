use bevy::prelude::*;
use crate::animation::systems::{
    animation_executor, animation_on_added_component, animation_on_removed_component,
};
use crate::in_game_state::InGameState;
use crate::movement::linear_movement::components::LinearMovementComponent;
use crate::player::animation::idle_animation::IdleAnimation;
use crate::player::animation::walking_animation::WalkingAnimation;
use crate::player::components::{Player};

pub mod components;
pub mod systems;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            animation_on_added_component::<Player, WalkingAnimation, LinearMovementComponent>
                .run_if(in_state(InGameState::InGame)),
        )
        .add_system(
            animation_executor::<Player, WalkingAnimation>.run_if(in_state(InGameState::InGame)),
        )
        .add_system(
            animation_on_removed_component::<Player, IdleAnimation, LinearMovementComponent>
                .run_if(in_state(InGameState::InGame)),
        );
    }
}
