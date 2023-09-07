use bevy::prelude::*;
use crate::game::world_map::audio::{CloseMapSoundEffect, OpenMapSoundEffect};
use crate::levels::components::{LevelTeleportClickedAudio, LevelTeleportHoveredAudio};
use crate::sound::systems::play_when_added;

pub mod systems;
pub mod typewriting;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(play_when_added::<LevelTeleportClickedAudio>)
            .add_system(play_when_added::<LevelTeleportHoveredAudio>)
            .add_system(play_when_added::<OpenMapSoundEffect>)
            .add_system(play_when_added::<CloseMapSoundEffect>);
    }
}
