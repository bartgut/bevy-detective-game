use bevy::prelude::*;
use crate::animation::systems::{
    animation_executor, animation_on_added_component, animation_on_removed_component,
};
use crate::movement::linear_movement::components::Linear2DMovementComponent;
use crate::npc::components::NPC;
use crate::npc::railwayman::animation::smoking_animation::SmokingAnimation;
use crate::player::animation::idle_animation::IdleAnimation;
use crate::player::animation::walking_animation::WalkingAnimation;
use crate::player::components::{Player};

pub mod components;
pub mod systems;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            animation_on_added_component::<Player, WalkingAnimation, Linear2DMovementComponent>,
        )
        .add_system(animation_on_added_component::<Player, IdleAnimation, Player>)
        .add_system(animation_executor::<Player, WalkingAnimation>)
        .add_system(
            animation_on_removed_component::<Player, IdleAnimation, Linear2DMovementComponent>,
        )
        .add_system(animation_on_added_component::<NPC, SmokingAnimation, NPC>)
        .add_system(animation_executor::<NPC, SmokingAnimation>);
    }
}
