use bevy::prelude::*;
use crate::comics::systems::{
    comics_cleanup, comics_inserted, comics_next_frame, multi_page_comics_inserted,
    multi_page_comics_next_page,
};
use crate::comics::vertical2images::components::Vertical2Images;
use crate::comics_state::ComicsState;

pub mod components;
pub mod systems;
pub mod vertical2images;

pub struct ComicsPlugin;

impl Plugin for ComicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(comics_inserted::<Vertical2Images>)
            .add_system(comics_cleanup.in_schedule(OnExit(ComicsState::ONGOING)))
            .add_system(comics_next_frame::<Vertical2Images>.in_set(OnUpdate(ComicsState::ONGOING)))
            .add_system(multi_page_comics_inserted::<Vertical2Images>)
            .add_system(multi_page_comics_next_page::<Vertical2Images>);
    }
}
