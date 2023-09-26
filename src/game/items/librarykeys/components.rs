use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::clickable::components::Clickable;
use crate::clickable::items::components::Collectible;
use crate::clickable::items::onesideitem::OneSideItem;
use crate::spawnable::components::SpawnableChild;

#[derive(Component)]
pub struct LibraryKeys;

impl SpawnableChild for LibraryKeys {
    fn spawn_child(&self, level: &mut EntityCommands, asset_server: &Res<AssetServer>) {
        level.with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: asset_server
                        .load(format!("images/items/librarykey/library_key_mini.png")),
                    transform: Transform::from_translation(Vec3::new(-600.0, -120.0, 1.0)),
                    ..default()
                },
                Clickable {
                    level_initial_position: Vec3::new(-800.0, -120.0, 1.0),
                    required_distance: 150.0,
                },
                Collectible {
                    inventory_sprite: "NONE.jpg".to_string(),
                    name: "library_keys".to_string(),
                    description: "Library keys".to_string(),
                },
                OneSideItem {
                    texture_file: "librarykey/library_key.png".to_string(),
                    sprite_entity: None,
                    click_sound: Some("items/key_grabbing.ogg".to_string()),
                },
            ));
        });
    }
}
