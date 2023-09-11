use bevy::prelude::*;
use crate::game::world_map::audio::{CloseMapSoundEffect, OpenMapSoundEffect};
use crate::levels::components::{LevelTeleportClickedAudio, LevelTeleportHoveredAudio};
use crate::sound::systems::play_when_added;
use crate::sound::typewriting::components::TypewriterClickedSoundEffect;

pub mod components;
pub mod systems;
pub mod typewriting;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, play_when_added::<LevelTeleportClickedAudio>)
            .add_systems(Update, play_when_added::<LevelTeleportHoveredAudio>)
            .add_systems(Update, play_when_added::<OpenMapSoundEffect>)
            .add_systems(Update, play_when_added::<CloseMapSoundEffect>)
            .add_systems(Update, play_when_added::<TypewriterClickedSoundEffect>);
    }
}
