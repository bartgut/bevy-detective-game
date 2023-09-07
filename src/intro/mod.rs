use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;
use crate::game_state::GameState;
use crate::intro_state::IntroState;

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Intro), start_intro)
            .add_systems(
                OnExit(IntroState::TypewritingReport),
                full_text_typewriting_cleanup,
            )
            .add_systems(OnEnter(IntroState::Comics1), comics_start)
            .add_systems(Update, mouse_interaction.run_if(in_state(GameState::Intro)));
    }
}
