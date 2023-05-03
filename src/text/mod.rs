use bevy::prelude::Plugin;
use bevy::prelude::*;
use crate::text::typewriting::systems::*;

pub mod typewriting;

pub struct TypeWritingTextPlugin;

impl Plugin for TypeWritingTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(type_writing_len_update)
            .add_system(type_writing_text_update);
    }
}
