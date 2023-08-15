use bevy::prelude::*;
use crate::comics::systems::{comics_next_frame, multi_page_comics_inserted};
use crate::comics_state::MultiPageComicsState;

pub mod components;
mod systems;
pub mod vertical2images;

pub struct ComicsPlugin;

impl Plugin for ComicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(comics_next_frame.in_set(OnUpdate(MultiPageComicsState::ONGOING)))
            .add_system(multi_page_comics_inserted);
    }
}
