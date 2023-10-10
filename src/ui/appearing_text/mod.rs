use bevy::app::{App, Plugin, Update};
use crate::ui::appearing_text::systems::{
    appearing_setup, cleanup_setup, disappearing_setup, not_visible_setup, visible_setup,
};

pub mod components;
pub mod systems;

pub struct AppearingTextPlugin;

impl Plugin for AppearingTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                not_visible_setup,
                appearing_setup,
                disappearing_setup,
                visible_setup,
                cleanup_setup,
            ),
        );
    }
}
