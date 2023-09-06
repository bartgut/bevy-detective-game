use bevy::prelude::*;
use crate::levels::components::{LevelTeleportClickedAudio, LevelTeleportHoveredAudio};
use crate::sound::systems::play_when_added;

pub mod systems;
pub mod typewriting;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(play_when_added::<LevelTeleportClickedAudio>);
        app.add_system(play_when_added::<LevelTeleportHoveredAudio>);
    }
}
