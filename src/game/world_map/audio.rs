use bevy::audio::PlaybackMode::Despawn;
use bevy::prelude::*;
use crate::sound::components::AudioPlayable;

#[derive(Component)]
pub struct OpenMapSoundEffect;

impl AudioPlayable for OpenMapSoundEffect {
    fn play(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands.spawn({
            AudioBundle {
                source: asset_server.load("sound/world_map/open_map.ogg"),
                settings: PlaybackSettings {
                    mode: Despawn,
                    ..default()
                },
                ..default()
            }
        });
    }
}

#[derive(Component)]
pub struct CloseMapSoundEffect;

impl AudioPlayable for CloseMapSoundEffect {
    fn play(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands.spawn({
            AudioBundle {
                source: asset_server.load("sound/world_map/close_map.ogg"),
                settings: PlaybackSettings {
                    mode: Despawn,
                    ..default()
                },
                ..default()
            }
        });
    }
}
