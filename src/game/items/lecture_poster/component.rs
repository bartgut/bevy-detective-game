use bevy::asset::AssetServer;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::{BuildChildren, Component, Res};
use crate::spawnable::components::SpawnableChild;
use bevy::prelude::*;
use rive_bevy::GenericEvent;
use crate::clickable::components::Clickable;
use crate::clickable::items::event_handler::EventHandler;
use crate::clickable::items::rive_item::RiveItem;
use crate::dialogs::dialog_runner::components::DialogEvent::Dialog;
use crate::dialogs::dialog_runner::components::DialogEventBundle;
use crate::dialogs::dialog_runner::components::DialogEventOwnership::TIMER;
use crate::global_state::global_state::UpdateGlobalState;

#[derive(Component)]
pub struct LecturePoster;

impl SpawnableChild for LecturePoster {
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
                RiveItem {
                    rive_file: asset_server.load("rive/main_menu.riv"),
                    artboard_name: "Flyer".to_string(),
                    animation_state_machine: "FlyerStateMachine".to_string(),
                    entity: None,
                },
                EventHandler::<GenericEvent> {
                    event_handle: |commands, event| {
                        if event.name == "NameClicked" {
                            commands.spawn(DialogEventBundle {
                                event: Dialog {
                                    speaker: "Player".into(),
                                    text: "Nadia.. ciekawe, mam nadzieje, że nie wyjechala jeszcze z miasta".into(),
                                    tags: vec![],
                                },
                                ownership: TIMER(5.0),
                            });
                            commands.spawn(UpdateGlobalState("lecturer_name_known".into(), true));
                        }

                        if event.name == "DateClicked" {
                            commands.spawn(DialogEventBundle {
                                event: Dialog {
                                    speaker: "Player".into(),
                                    text: "20 maja, dzien przed zaginieciem pociagu".into(),
                                    tags: vec![],
                                },
                                ownership: TIMER(5.0),
                            });
                            commands.spawn(UpdateGlobalState("lecture_date_known".into(), true));
                        }
                    },
                },
            ));
        });
    }
}
