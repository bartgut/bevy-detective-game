use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::clickable::components::Clickable;
use crate::clickable::items::twosideitem::TwoSideItem;
use crate::spawnable::components::SpawnableChild;

#[derive(Component)]
pub struct LoverPhoto;

impl SpawnableChild for LoverPhoto {
    fn spawn_child(&self, level: &mut EntityCommands, asset_server: &Res<AssetServer>) {
        level.with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: asset_server.load(format!("images/items/letter/young_lover_mini.png")),
                    transform: Transform::from_translation(Vec3::new(-800.0, -120.0, 1.0)),
                    ..default()
                },
                Clickable {
                    level_initial_position: Vec3::new(-800.0, -120.0, 1.0),
                    required_distance: 150.0,
                },
                TwoSideItem::builder()
                    .texture_front_file("letter/young_lover_letter_front.png".to_string())
                    .texture_back_file("letter/young_lover_letter_back.png".to_string())
                    .with_dialog((
                        "Player".to_string(),
                        "Wydaje sie troche za mloda jak na zone nieboszczyka".to_string(),
                    ))
                    .with_state_update(("found_young_lover_photo".to_string(), true))
                    .build(),
            ));
        });
    }
}
