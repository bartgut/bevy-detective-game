use bevy::prelude::*;
use crate::ui::appearing_text::AppearingTextPlugin;
use crate::ui::systems::{button_interaction_hover_handle, invisible_to_visible_transition};

pub mod appearing_text;
pub mod components;
pub mod systems;

pub struct UIUtilsPlugin;

impl Plugin for UIUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, invisible_to_visible_transition)
            .add_systems(Update, button_interaction_hover_handle::<Text>)
            .add_plugins(AppearingTextPlugin);
    }
}
