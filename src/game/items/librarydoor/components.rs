use bevy::audio::PlaybackMode::Despawn;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::clickable::components::{Clickable, ClickConditions};
use crate::inventory::components::Inventory;
use crate::level_state::LevelState;
use crate::levels::components::LevelTeleport;
use crate::spawnable::components::SpawnableChild;
use crate::clickable::components::ClickCondition::InventoryCondition;
use crate::dialogs::dialog_runner::components::{DialogEvent, DialogEventBundle};
use crate::dialogs::dialog_runner::components::DialogEventOwnership::TIMER;

#[derive(Component)]
pub struct LibraryDoor;

impl SpawnableChild for LibraryDoor {
    fn spawn_child(&self, level: &mut EntityCommands, _: &Res<AssetServer>) {
        level.with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 0.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(70.0, 150.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(90.0, 0.0, 1.0)),
                    ..default()
                },
                Clickable {
                    required_distance: 150.0,
                },
                ClickConditions {
                    condition: vec![InventoryCondition {
                        0: |inventory| inventory.has_item("library_keys"),
                    }],
                    failure: |commands, asset_server| {
                        commands.spawn(DialogEventBundle {
                            event: DialogEvent::Dialog {
                                speaker: "Player".to_string(),
                                text: "Zamkniete, potrzebuje klucza".to_string(),
                                tags: vec![],
                            },
                            ownership: TIMER(3.0),
                        });
                        commands.spawn(AudioBundle {
                            source: asset_server.load(format!("sound/items/door_locked.ogg")),
                            settings: PlaybackSettings {
                                mode: Despawn,
                                ..default()
                            },
                            ..default()
                        });
                    },
                },
                LevelTeleport {
                    level_state: LevelState::TicketOffice,
                },
            ));
        });
    }
}
