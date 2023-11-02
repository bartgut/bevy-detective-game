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
    pub used: bool,
}
