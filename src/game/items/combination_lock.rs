use bevy::asset::AssetServer;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::{BuildChildren, Component, Res};
use crate::spawnable::components::SpawnableChild;
use bevy::prelude::*;
use bevy_yarnspinner::dialog_runner::components::DialogEvent::Dialog;
use bevy_yarnspinner::dialog_runner::components::DialogEventBundle;
use bevy_yarnspinner::dialog_runner::components::DialogEventOwnership::TIMER;
use rive_bevy::GenericEvent;
use crate::clickable::components::Clickable;
use crate::clickable::items::event_handler::EventHandler;
use crate::clickable::items::rive_item::RiveItem;
use crate::global_state::global_state::UpdateGlobalState;

#[derive(Component)]
pub struct CombinationLock;

impl SpawnableChild for CombinationLock {
    fn spawn_child(&self, level: &mut EntityCommands, asset_server: &Res<AssetServer>) {
        level.with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 0.0, 0.0, 1.0),
                        custom_size: Some(Vec2::new(110.0, 110.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(200.0, 0.0, 1.0)),
                    ..default()
                },
                Clickable {
                    required_distance: 150.0,
                },
                RiveItem {
                    rive_file: asset_server.load("rive/combination_lock.riv"),
                    artboard_name: "CombinationLock".to_string(),
                    animation_state_machine: "CombinationLockMachine".to_string(),
                    entity: None,
                },
                EventHandler::<GenericEvent> {
                    event_handle: |commands, event| {
                        if event.name == "WrongCodeSet" {
                            commands.spawn(DialogEventBundle {
                                event: Dialog {
                                    speaker: "Player".into(),
                                    text: "Zly kod".into(),
                                    tags: vec![],
                                },
                                ownership: TIMER(3.0),
                            });
                        }

                        if event.name == "CorrectCodeSet" {
                            commands.spawn(DialogEventBundle {
                                event: Dialog {
                                    speaker: "Player".into(),
                                    text: "Poprawny kod".into(),
                                    tags: vec![],
                                },
                                ownership: TIMER(3.0),
                            });
                            commands.spawn(UpdateGlobalState(
                                "monitoring_room_door_unlocked".into(),
                                true,
                            ));
                        }
                    },
                },
            ));
        });
    }
}
