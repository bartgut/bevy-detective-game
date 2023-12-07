use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use rive_bevy::{Riv, SceneTarget, SpriteEntity, StateMachine};
use crate::clickable::items::behaviour::ClickableBehaviour;

#[derive(Component)]
pub struct RiveItem {
    pub rive_file: Handle<Riv>,
    pub artboard_name: String,
    pub animation_state_machine: String,
    pub entity: Option<Entity>,
}

impl ClickableBehaviour for RiveItem {
    fn on_hover_entry(&mut self, commands: &mut Commands) {}

    fn on_start(&mut self, commands: &mut Commands, asset_server: Res<AssetServer>) {
        let mut item_image = Image::default();
        item_image.resize(Extent3d {
            width: 1280,
            height: 960,
            ..default()
        });
        let animation_image_handle = asset_server.add(item_image);

        let machine = StateMachine {
            riv: self.rive_file.clone_weak(),
            handle: rive_bevy::Handle::Name(self.animation_state_machine.clone().into()),
            artboard_handle: rive_bevy::Handle::Name(self.artboard_name.clone().into()),
            ..default()
        };

        self.entity = Some(
            commands
                .spawn(SpriteBundle {
                    texture: animation_image_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 999.0))
                        .with_scale(Vec3::splat(0.6)),
                    ..default()
                })
                .insert(machine)
                .id(),
        );

        commands.entity(self.entity.unwrap()).insert(SceneTarget {
            image: animation_image_handle.clone(),
            sprite: SpriteEntity {
                entity: self.entity,
            },
            ..default()
        });
    }

    fn on_click(&mut self, commands: &mut Commands, _asset_server: Res<AssetServer>) {}

    fn on_close(&mut self, commands: &mut Commands) {
        commands.entity(self.entity.unwrap()).despawn();
    }
}
