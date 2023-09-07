use bevy::prelude::*;
use crate::comics::systems::{comics_next_frame, multi_page_comics_inserted};
use crate::comics_state::MultiPageComicsState;

pub mod components;
mod systems;
pub mod vertical2images;

pub struct ComicsPlugin;

impl Plugin for ComicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            comics_next_frame.run_if(in_state(MultiPageComicsState::ONGOING)),
        )
        .add_systems(Update, multi_page_comics_inserted);
    }
}
