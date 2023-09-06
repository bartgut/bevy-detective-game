use super::behaviour::ClickableBehaviour;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct OneSideItem {
    pub texture_file: String,
    pub spriteEntity: Option<Entity>,
}

impl ClickableBehaviour for OneSideItem {
    fn on_hover_entry(&mut self, commands: &mut Commands) {}

    fn on_start(&mut self, commands: &mut Commands, asset_server: Res<AssetServer>) {
        let x = commands.spawn(
            (SpriteBundle {
                texture: asset_server.load(format!("images/items/{}", self.texture_file)),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 999.0),
                    scale: Vec3::new(0.3, 0.3, 0.3),
                    ..Default::default()
                },
                ..default()
            }),
        );
        self.spriteEntity = Some(x.id());
    }

    fn on_click(&mut self, commands: &mut Commands, asset_server: Res<AssetServer>) {}

    fn on_close(&mut self, commands: &mut Commands) {
        commands.entity(self.spriteEntity.unwrap()).despawn();
    }
}
