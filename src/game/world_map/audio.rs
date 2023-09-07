use bevy::prelude::*;
use crate::sound::typewriting::components::AudioPlayable;

#[derive(Component)]
pub struct OpenMapSoundEffect;

impl AudioPlayable for OpenMapSoundEffect {
    fn play(&self, asset_server: &Res<AssetServer>, audio: &Res<Audio>) {
        audio.play(asset_server.load("sound/world_map/open_map.ogg"));
    }
}

#[derive(Component)]
pub struct CloseMapSoundEffect;

impl AudioPlayable for CloseMapSoundEffect {
    fn play(&self, asset_server: &Res<AssetServer>, audio: &Res<Audio>) {
        audio.play(asset_server.load("sound/world_map/close_map.ogg"));
    }
}
