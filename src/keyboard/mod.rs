use bevy::app::{App, Plugin, Update};
use crate::keyboard::system::keyboard_event;

mod system;

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_event);
    }
}
