use bevy::app::Plugin;
use bevy::prelude::*;
use crate::comics::rive::systems::{comics_interaction_events, despawn_comics, initialize_comics};
use crate::intro_state::IntroState::Comics1;

pub mod components;
mod systems;

pub struct RiveComicsPlugin;

impl Plugin for RiveComicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, initialize_comics)
            .add_systems(Update, comics_interaction_events)
            .add_systems(OnExit(Comics1), despawn_comics);
    }
}
