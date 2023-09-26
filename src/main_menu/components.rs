use bevy::audio::PlaybackMode::Despawn;
use bevy::prelude::*;
use crate::sound::components::UIInteractionSoundEffect;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct MainMenuImage;

#[derive(Component)]
pub struct MainMenuButton;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct StartGameText;

#[derive(Component)]
pub struct QuitGameButton;

#[derive(Component)]
pub struct QuitGameText;

impl UIInteractionSoundEffect for MainMenuButton {
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
