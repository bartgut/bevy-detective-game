use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::clickable::components::Clickable;
use crate::clickable::items::components::Collectible;
use crate::clickable::items::onesideitem::OneSideItem;
use crate::spawnable::components::SpawnableChild;

#[derive(Component)]
pub struct EgyptianNecklace;

impl SpawnableChild for EgyptianNecklace {
    fn spawn_child(&self, level: &mut EntityCommands, asset_server: &Res<AssetServer>) {
        level.with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 0.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(32.0, 32.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(210.0, -130.0, 1.0)),
                    ..default()
                },
                Clickable {
                    required_distance: 150.0,
                },
                OneSideItem {
                    texture_file: "egyptian_necklace/egyptian_necklace.png".to_string(),
                    sprite_entity: None,
                    click_sound: None,
                },
            ));
        });
    }
}
