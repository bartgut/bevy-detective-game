use bevy::prelude::*;
use crate::animation::systems::{
    animation_executor, animation_on_added_component, animation_on_removed_component,
};
use crate::game::effects::train_smoke::{TrainSmoke, TrainSmokeAnimation};
use crate::game::npc::railwayman::animation::smoking_animation::SmokingAnimation;
use crate::movement::linear_movement::components::Linear2DMovementComponent;
use crate::npc::components::NPC;
use crate::player::animation::idle_animation::IdleAnimation;
use crate::player::animation::walking_animation::WalkingAnimation;
use crate::player::components::{Player};

pub mod components;
pub mod systems;

pub struct SpriteAnimationPlugin;

macro_rules! add_animation {
    ($app:expr, $comp:ty, $anim:ty) => {
        $app.add_systems(Update, animation_on_added_component::<$comp, $anim, $comp>)
            .add_systems(Update, animation_executor::<$comp, $anim>)
    };
}

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            animation_on_added_component::<Player, WalkingAnimation, Linear2DMovementComponent>,
        )
        .add_systems(
            Update,
            animation_on_added_component::<Player, IdleAnimation, Player>,
        )
        .add_systems(Update, animation_executor::<Player, WalkingAnimation>)
        .add_systems(
            Update,
            animation_on_removed_component::<Player, IdleAnimation, Linear2DMovementComponent>,
        );
        add_animation!(app, TrainSmoke, TrainSmokeAnimation);
        add_animation!(app, NPC, SmokingAnimation);
    }
}
