use bevy::prelude::*;
use crate::animation::components::AnimationHandles;
use crate::animation::systems::{animation_executor, animation_on_added_component, animation_on_removed_component, animation_preload};
use crate::game::effects::train_smoke::{TrainSmoke, TrainSmokeAnimation};
use crate::game::npc::railwayman::animation::smoking_animation::SmokingAnimation;
use crate::game_state::GameState;
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
        $app.add_systems(Update, animation_on_added_component::<$comp, $anim, $comp>.run_if(in_state(GameState::InGame)))
            .add_systems(Update, animation_executor::<$comp, $anim>.run_if(in_state(GameState::InGame)))
    };
}

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimationHandles { ..default() }).add_systems(
            Update,
            animation_on_added_component::<Player, WalkingAnimation, Linear2DMovementComponent>
                .run_if(in_state(GameState::InGame))
        )
        .add_systems(
            Update,
            animation_on_added_component::<Player, IdleAnimation, Player>
                .run_if(in_state(GameState::InGame))
        )
        .add_systems(Update, animation_executor::<Player, WalkingAnimation>
            .run_if(in_state(GameState::InGame))
        )
        .add_systems(
            Update,
            animation_on_removed_component::<Player, IdleAnimation, Linear2DMovementComponent>
            .run_if(in_state(GameState::InGame))
        ).add_systems(OnExit(GameState::InLevelSpritesLoading), animation_preload::<WalkingAnimation>)
            .add_systems(OnExit(GameState::InLevelSpritesLoading), animation_preload::<IdleAnimation>)
            .add_systems(OnExit(GameState::InLevelSpritesLoading), animation_preload::<TrainSmokeAnimation>)
            .add_systems(OnExit(GameState::InLevelSpritesLoading), animation_preload::<SmokingAnimation>);
        add_animation!(app, TrainSmoke, TrainSmokeAnimation);
        add_animation!(app, NPC, SmokingAnimation);
    }
}
