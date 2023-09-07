use bevy::audio::PlaybackMode::Despawn;
use bevy::prelude::*;
use crate::clickable::items::behaviour::ClickableBehaviour;
use crate::level_state::LevelState;
use crate::sound::components::AudioPlayable;

#[derive(Bundle, Clone)]
pub struct LevelBundle {
    pub level_description: LevelDescription,
    pub transform: Transform,
}

#[derive(Component, Clone)]
pub struct LevelDescription {
    pub level_name: String,
    pub player_initial_position: Transform,
}

#[derive(Component)]
pub struct CurrentLevel;

#[derive(Component)]
pub struct CurrentLevelSprite;

#[derive(Component)]
pub struct LevelChangeTrigger {
    pub level_state: LevelState,
}

#[derive(Component)]
pub struct LevelTeleport {
    pub level_state: LevelState,
}

#[derive(Component)]
pub struct LevelTeleportClickedAudio;
impl AudioPlayable for LevelTeleportClickedAudio {
    fn play(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands.spawn({
            AudioBundle {
                source: asset_server.load("sound/world_map/level_teleport_clicked.ogg"),
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
pub struct LevelTeleportHoveredAudio;

impl AudioPlayable for LevelTeleportHoveredAudio {
    fn play(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands.spawn({
            AudioBundle {
                source: asset_server.load("sound/world_map/level_teleport_hover.ogg"),
                settings: PlaybackSettings {
                    mode: Despawn,
                    ..default()
                },
                ..default()
            }
        });
    }
}

impl ClickableBehaviour for LevelTeleport {
    fn on_hover_entry(&mut self, commands: &mut Commands) {
        commands.spawn(LevelTeleportHoveredAudio);
    }

    fn on_start(&mut self, commands: &mut Commands, _: Res<AssetServer>) {
        commands.spawn((
            LevelChangeTrigger {
                level_state: self.level_state.clone(),
            },
            LevelTeleportClickedAudio,
        ));
    }

    fn on_click(&mut self, _: &mut Commands, _: Res<AssetServer>) {}

    fn on_close(&mut self, _: &mut Commands) {}
}
