use bevy::prelude::*;
use crate::comics::systems::{
    comics_page_inserted, comics_page_mouse_interaction, multi_page_comics_inserted,
    multi_page_comics_next_page,
};

pub mod components;
pub mod config;
pub mod renderer;
pub mod rive;
mod systems;

pub struct ComicsPlugin;

impl Plugin for ComicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, comics_page_inserted)
            .add_systems(Update, comics_page_mouse_interaction)
            .add_systems(Update, multi_page_comics_inserted)
            .add_systems(Update, multi_page_comics_next_page);
    }
}
