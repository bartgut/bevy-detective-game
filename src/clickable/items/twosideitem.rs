use bevy::audio::PlaybackMode::Remove;
use super::behaviour::ClickableBehaviour;
use bevy::prelude::*;
use bevy_yarnspinner::dialog_runner::components::DialogEvent::Dialog;
use bevy_yarnspinner::dialog_runner::components::DialogEventBundle;
use bevy_yarnspinner::dialog_runner::components::DialogEventOwnership::PARENT;
use crate::global_state::global_state::UpdateGlobalState;

#[derive(Clone)]
pub enum TextureSide {
    Front,
    Back,
}

#[derive(Component, Clone)]
pub struct TwoSideItem {
    pub texture_front_file: String,
    pub texture_back_file: String,
    pub additional_dialog: Option<(String, String)>,
    pub current_texture_sprite: Option<Entity>,
    pub state_update: Option<(String, bool)>,
    current_texture_site: TextureSide,
    dialog_entity: Option<Entity>,
    pub click_sound: Option<String>,
}

#[buildstructor::buildstructor]
impl TwoSideItem {
    #[builder(visibility = "pub")]
    fn new(
        texture_front_file: String,
        texture_back_file: String,
        with_dialog: Option<(String, String)>,
        with_state_update: Option<(String, bool)>,
        with_click_sound: Option<String>,
    ) -> Self {
        TwoSideItem {
            texture_front_file: texture_front_file,
            texture_back_file: texture_back_file,
            additional_dialog: with_dialog,
            state_update: with_state_update,
            current_texture_sprite: None,
            current_texture_site: TextureSide::Front,
            dialog_entity: None,
            click_sound: with_click_sound,
        }
    }

    fn play_sound_if_needed(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        match &self.click_sound {
            Some(sound) => {
                commands.spawn(AudioBundle {
                    source: asset_server.load(format!("sound/{}", sound)),
                    settings: PlaybackSettings {
                        mode: Remove,
                        ..default()
                    },
                    ..default()
                });
            }
            _ => {}
        }
    }

    fn show_dialog_if_needed(&mut self, commands: &mut Commands) {
        match &self.additional_dialog {
            Some((subject, text)) => {
                self.dialog_entity = Some(
                    commands
                        .spawn(DialogEventBundle {
                            event: Dialog {
                                speaker: subject.clone(),
                                text: text.clone(),
                                tags: vec![],
                            },
                            ownership: PARENT,
                        })
                        .id(),
                )
            }
            None => {}
        }
    }
}

impl ClickableBehaviour for TwoSideItem {
    fn on_hover_entry(&mut self, _: &mut Commands) {}

    fn on_start(&mut self, commands: &mut Commands, asset_server: Res<AssetServer>) {
        let what_texture = match self.current_texture_site {
            TextureSide::Front => &self.texture_front_file,
            TextureSide::Back => &self.texture_back_file,
        };

        let x = commands
            .spawn(SpriteBundle {
                texture: asset_server.load(format!("images/items/{}", what_texture)),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 999.0),
                    scale: Vec3::new(0.3, 0.3, 0.3),
                    ..Default::default()
                },
                ..default()
            })
            .id();
        self.current_texture_sprite = Some(x);
        if let Some((key, value)) = &self.state_update {
            commands.spawn(UpdateGlobalState(key.clone(), *value));
        }
        self.show_dialog_if_needed(commands);
        self.play_sound_if_needed(commands, &asset_server);
    }

    fn on_click(&mut self, commands: &mut Commands, asset_server: Res<AssetServer>) {
        let what_texture = match self.current_texture_site {
            TextureSide::Front => {
                self.current_texture_site = TextureSide::Back;
                &self.texture_back_file
            }
            TextureSide::Back => {
                self.current_texture_site = TextureSide::Front;
                &self.texture_front_file
            }
        };
        commands
            .entity(self.current_texture_sprite.unwrap())
            .despawn();

        let x = commands.spawn(SpriteBundle {
            texture: asset_server.load(format!("images/items/{}", what_texture)),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 999.0),
                scale: Vec3::new(0.3, 0.3, 0.3),
                ..Default::default()
            },
            ..default()
        });
        self.current_texture_sprite = Some(x.id());
    }

    fn on_close(&mut self, commands: &mut Commands) {
        if let Some(dialog_entity) = self.dialog_entity {
            commands.entity(dialog_entity).despawn_recursive();
        }
        if let Some(current_texture_sprite) = self.current_texture_sprite {
            commands.entity(current_texture_sprite).despawn();
        }
    }
}
