use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;
use crate::assets::asset_loading_monitor::AssetLoadingStateChangeExt;
use crate::comics::rive::components::RiveComicsSink;
use crate::game_state::GameState;
use crate::intro_state::IntroState;
use crate::intro_state::IntroState::Comics1Loading;

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Intro), start_intro)
            .add_systems(
                OnExit(IntroState::TypewritingReport),
                full_text_typewriting_cleanup,
            )
            .add_systems(OnEnter(Comics1Loading), comics_start)
            .add_component_loading_state::<RiveComicsSink, IntroState>(
                Comics1Loading,
                IntroState::Comics1,
            )
            .add_systems(Update, mouse_interaction.run_if(in_state(GameState::Intro)));
    }
}
