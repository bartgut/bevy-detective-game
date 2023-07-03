use bevy::prelude::*;
use crate::level_state::LevelState;

pub mod components;
pub mod systems;

use systems::*;
use crate::game_state::GameState;
use crate::intro_state::IntroState;

pub struct IntroPlugin;

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub struct IntroInteractions;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_intro.in_schedule(OnEnter(GameState::Intro)))
            .add_system(
                full_text_typewriting_cleanup.in_schedule(OnExit(IntroState::TypewritingReport)),
            )
            .add_system(comics_start.in_schedule(OnEnter(IntroState::Comics1)))
            .add_system(
                mouse_interaction
                    .in_set(IntroInteractions)
                    .in_set(OnUpdate(GameState::Intro)),
            );
    }
}
