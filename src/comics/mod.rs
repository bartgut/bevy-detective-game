use bevy::prelude::*;
use crate::comics::systems::{comics_cleanup, comics_inserted, comics_next_frame};
use crate::comics::vertical2images::components::Vertical2Images;
use crate::comics_state::ComicsState;
use crate::ui::systems::invisible_to_visible_transition;

pub mod components;
pub mod systems;
pub mod vertical2images;

pub struct ComicsPlugin;

impl Plugin for ComicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(comics_inserted::<Vertical2Images>)
            .add_system(comics_cleanup.in_schedule(OnExit(ComicsState::ONGOING)))
            .add_system(
                comics_next_frame::<Vertical2Images>.in_set(OnUpdate(ComicsState::ONGOING)),
            );
    }
}
