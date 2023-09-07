use super::behaviour::ClickableBehaviour;
use bevy::prelude::*;
use crate::dialogs::ui::systems::build_dialog_ui;

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
    current_texture_sprite: Option<Entity>,
    current_texture_site: TextureSide,
    dialog_entity: Option<Entity>,
}

impl TwoSideItem {
    pub fn new_no_dialog(texture_front_file: String, texture_back_file: String) -> Self {
        TwoSideItem {
            texture_front_file: texture_front_file,
            texture_back_file: texture_back_file,
            additional_dialog: None,
            current_texture_sprite: None,
            current_texture_site: TextureSide::Front,
            dialog_entity: None,
        }
    }

    pub fn new_with_dialog(
        texture_front_file: String,
        texture_back_file: String,
        additional_dialog_subject: String,
        additional_dialog_text: String,
    ) -> Self {
        TwoSideItem {
            texture_front_file: texture_front_file,
            texture_back_file: texture_back_file,
            additional_dialog: Some((additional_dialog_subject, additional_dialog_text)),
            current_texture_sprite: None,
            current_texture_site: TextureSide::Front,
            dialog_entity: None,
        }
    }

    fn show_dialog_if_needed(&mut self, commands: &mut Commands, asset_server: Res<AssetServer>) {
        match &self.additional_dialog {
            Some((subject, text)) => {
                self.dialog_entity = Some(build_dialog_ui(commands, asset_server, &subject, &text));
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
        self.show_dialog_if_needed(commands, asset_server);
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
