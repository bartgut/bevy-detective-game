use bevy::audio::PlaybackMode::Despawn;
use bevy::prelude::*;
use crate::sound::components::UIInteractionSoundEffect;

#[derive(Component)]
pub struct DialogUI;

#[derive(Component)]
pub struct DialogUIText;

#[derive(Component)]
pub struct DialogUIImage;

#[derive(Component)]
pub struct OptionUI;

#[derive(Component)]
pub struct OptionUINode {
    pub node_title: String,
}

impl UIInteractionSoundEffect for OptionUINode {
    fn on_hover(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands.spawn(AudioBundle {
            source: asset_server.load(format!("sound/ui/button_hover.ogg")),
            settings: PlaybackSettings {
                mode: Despawn,
                ..default()
            },
            ..default()
        });
    }

    fn on_pressed(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands.spawn(AudioBundle {
            source: asset_server.load(format!("sound/ui/button_clicked.ogg")),
            settings: PlaybackSettings {
                mode: Despawn,
                ..default()
            },
            ..default()
        });
    }
}
