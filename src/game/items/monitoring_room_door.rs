use bevy::audio::PlaybackMode::Despawn;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_yarnspinner::dialog_runner::components::{DialogEvent, DialogEventBundle};
use bevy_yarnspinner::dialog_runner::components::DialogEventOwnership::TIMER;
use bevy_yarnspinner::dialog_runner::context::StateContext;
use crate::clickable::components::{Clickable, ClickCondition, ClickConditions};
use crate::global_state::global_state::UpdateGlobalState;
use crate::level_state::LevelState;
use crate::levels::components::LevelTeleport;
use crate::spawnable::components::SpawnableChild;

#[derive(Component)]
pub struct MonitoringRoomDoor;

impl SpawnableChild for MonitoringRoomDoor {
    fn spawn_child(&self, level: &mut EntityCommands, _: &Res<AssetServer>) {
        level.with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 0.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(110.0, 240.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(20.0, -20.0, 1.0)),
                    ..default()
                },
                Clickable {
                    required_distance: 150.0,
                },
                ClickConditions {
                    condition: vec![ClickCondition::StateCondition {
                        0: |state| {
                            state
                                .get_value("monitoring_room_door_unlocked")
                                .map_or(false, |value| *value)
                        },
                    }],
                    failure: |commands, asset_server| {
                        commands.spawn(DialogEventBundle {
                            event: DialogEvent::Dialog {
                                speaker: "Player".to_string(),
                                text: "Zamkniete, musze wpisac poprawny kod".to_string(),
                                tags: vec![],
                            },
                            ownership: TIMER(6.0),
                        });
                        commands.spawn(UpdateGlobalState("library_doors_locked".to_string(), true));
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
