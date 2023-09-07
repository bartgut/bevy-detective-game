use bevy::prelude::*;
use crate::ui::systems::invisible_to_visible_transition;

pub mod components;
pub mod systems;

pub struct UIUtilsPlugin;

impl Plugin for UIUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, invisible_to_visible_transition);
    }
}
