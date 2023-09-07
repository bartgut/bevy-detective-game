use bevy::audio::PlaybackMode::Despawn;
use bevy::prelude::*;
use crate::sound::components::AudioPlayable;

#[derive(Component)]
pub struct TypewriterClickedSoundEffect {
    pub chosen_letter: i32,
}

impl AudioPlayable for TypewriterClickedSoundEffect {
    fn play(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands.spawn({
            AudioBundle {
                source: asset_server.load(format!(
                    "sound/typewriting/typewriting{}.ogg",
                    self.chosen_letter
                )),
                settings: PlaybackSettings {
                    mode: Despawn,
                    ..default()
                },
                ..default()
            }
        });
    }
}
