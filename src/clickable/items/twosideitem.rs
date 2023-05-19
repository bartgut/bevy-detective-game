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
    pub current_texture_sprite: Option<Entity>,
    pub current_texture_site: TextureSide,
    pub dialog_entity: Option<Entity>,
}

impl ClickableBehaviour for TwoSideItem {
    fn on_start(&mut self, commands: &mut Commands, asset_server: Res<AssetServer>) {
        let what_texture = match self.current_texture_site {
            TextureSide::Front => &self.texture_front_file,
            TextureSide::Back => &self.texture_back_file,
        };

        let x = commands.spawn(
            (SpriteBundle {
                texture: asset_server.load(format!("images/items/{}", what_texture)),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 999.0),
                    scale: Vec3::new(0.3, 0.3, 0.3),
                    ..Default::default()
                },
                ..default()
            }),
        );
        self.current_texture_sprite = Some(x.id());
        self.dialog_entity = Some(build_dialog_ui(
            commands,
            asset_server,
            &"Player".to_string(),
            &"Troche za mloda jak na zone".to_string(),
        ));
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

        let x = commands.spawn(
            (SpriteBundle {
                texture: asset_server.load(format!("images/items/{}", what_texture)),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 999.0),
                    scale: Vec3::new(0.3, 0.3, 0.3),
                    ..Default::default()
                },
                ..default()
            }),
        );
        self.current_texture_sprite = Some(x.id());
    }

    fn on_close(&mut self, commands: &mut Commands) {
        commands
            .entity(self.current_texture_sprite.unwrap())
            .despawn();
        commands
            .entity(self.dialog_entity.unwrap())
            .despawn_recursive();
    }
}
