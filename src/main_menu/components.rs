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
