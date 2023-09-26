use bevy::audio::PlaybackMode::Despawn;
use super::behaviour::ClickableBehaviour;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct OneSideItem {
    pub texture_file: String,
    pub sprite_entity: Option<Entity>,
    pub click_sound: Option<String>,
}

impl OneSideItem {
    pub fn play_sound_if_needed(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) {
        match &self.click_sound {
            Some(sound) => {
                commands.spawn(AudioBundle {
                    source: asset_server.load(format!("sound/{}", sound)),
                    settings: PlaybackSettings {
                        mode: Despawn,
                        ..default()
                    },
                    ..default()
                });
            }
            _ => {}
        }
    }
}

impl ClickableBehaviour for OneSideItem {
    fn on_hover_entry(&mut self, _: &mut Commands) {}

    fn on_start(&mut self, commands: &mut Commands, asset_server: Res<AssetServer>) {
        let x = commands
            .spawn(SpriteBundle {
                texture: asset_server.load(format!("images/items/{}", self.texture_file)),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 999.0),
                    scale: Vec3::new(0.3, 0.3, 0.3),
                    ..Default::default()
                },
                ..default()
            })
            .id();
        self.sprite_entity = Some(x);
        self.play_sound_if_needed(commands, &asset_server)
    }

    fn on_click(&mut self, _: &mut Commands, _: Res<AssetServer>) {}

    fn on_close(&mut self, commands: &mut Commands) {
        commands.entity(self.sprite_entity.unwrap()).despawn();
    }
}
