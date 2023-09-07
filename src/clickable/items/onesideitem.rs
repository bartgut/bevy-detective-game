use super::behaviour::ClickableBehaviour;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct OneSideItem {
    pub texture_file: String,
    pub sprite_entity: Option<Entity>,
}

impl ClickableBehaviour for OneSideItem {
    fn on_hover_entry(&mut self, _: &mut Commands) {}

    fn on_start(&mut self, commands: &mut Commands, asset_server: Res<AssetServer>) {
        let x = commands.spawn(SpriteBundle {
            texture: asset_server.load(format!("images/items/{}", self.texture_file)),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 999.0),
                scale: Vec3::new(0.3, 0.3, 0.3),
                ..Default::default()
            },
            ..default()
        });
        self.sprite_entity = Some(x.id());
    }

    fn on_click(&mut self, _: &mut Commands, _: Res<AssetServer>) {}

    fn on_close(&mut self, commands: &mut Commands) {
        commands.entity(self.sprite_entity.unwrap()).despawn();
    }
}
