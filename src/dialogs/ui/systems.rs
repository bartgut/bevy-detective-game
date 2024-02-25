use bevy::audio::PlaybackMode::Remove;
use bevy::ecs::system::EntityCommands;
use super::components::*;
use bevy::prelude::*;
use bevy::prelude::TimerMode::Repeating;
use bevy_yarnspinner::asset::asset::YarnSpinnerDialog;
use bevy_yarnspinner::dialog_runner::components::{
    CurrentDialogEvent, DialogEvent, DialogEventBundle, DialogEventOwnership, DialogEventTimer,
    DialogOption,
};
use bevy_yarnspinner::dialog_runner::context::StateContext;
use bevy_yarnspinner::dialog_runner::runner::DialogRunner;
use crate::assets::fonts::Fonts;
use crate::clickable::components::Clicked;
use crate::dialogs::dialogs::resource::*;
use crate::dialogs::ui::dialog_internal_state::DialogInternalState;
use crate::global_state::global_state::GlobalState;
use crate::in_game_state::InGameState;
use crate::in_game_state::InGameState::Dialog;
use crate::npc::components::{DialogableNPC, NPCInDialog};
use crate::text::typewriting::systems::create_type_writing_text;
use crate::ui::components::ButtonInteractionAction;

pub fn dialog_ui_events_with_timer_ownership(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DialogEventTimer)>,
) {
    for (entity, mut timer) in query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn dialog_ui_from_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fonts: Res<Fonts>,
    mut avatars: ResMut<AvatarHandles>,
    query: Query<(Entity, &DialogEvent, &DialogEventOwnership), Added<DialogEvent>>,
    mut current_dialog_event: Query<Entity, With<CurrentDialogEvent>>,
) {
    for (entity, event, ownership) in query.iter() {
        delete_current_dialog_event(&mut current_dialog_event, &mut commands, &event);
        let mut owning_entity = commands.entity(entity);

        if let DialogEventOwnership::TIMER(time) = ownership {
            owning_entity.insert(DialogEventTimer(Timer::from_seconds(
                *time,
                TimerMode::Once,
            )));
        }

        match event {
            DialogEvent::Dialog {
                speaker,
                text,
                tags,
            } => {
                build_dialog_ui(
                    &mut owning_entity,
                    &asset_server,
                    &mut avatars,
                    &fonts,
                    speaker,
                    text,
                );
                for tag in tags.iter() {
                    if tag.name == "audio" {
                        owning_entity.insert(AudioBundle {
                            source: asset_server
                                .load(format!("dialogs/audio/{}", tag.value.clone())),
                            settings: PlaybackSettings {
                                mode: Remove,
                                ..default()
                            },
                            ..default()
                        });
                    }
                }
            }
            DialogEvent::Options {
                speaker: _,
                options,
            } => build_options_ui(&mut owning_entity, &asset_server, &fonts, options),
            _ => {}
        }
    }
}

fn delete_current_dialog_event(
    query: &mut Query<Entity, With<CurrentDialogEvent>>,
    commands: &mut Commands,
    current_event: &DialogEvent,
) {
    if let Ok(current_dialog) = query.get_single() {
        if let DialogEvent::Waiting = current_event {
            return;
        }
        commands.entity(current_dialog).despawn_recursive();
    }
}

fn build_options_ui(
    owning_entity: &mut EntityCommands,
    asset_server: &Res<AssetServer>,
    fonts: &Res<Fonts>,
    options: &Vec<DialogOption>,
) {
    owning_entity
        .insert(CurrentDialogEvent)
        .insert((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(20.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    column_gap: Val::Px(8.0),
                    top: Val::Percent(80.0),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            OptionUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(160.0),
                        height: Val::Px(160.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: asset_server.load("images/avatars/Player.png").into(),
                    ..default()
                },
                DialogUIImage,
            ));
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Auto,
                            height: Val::Auto,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(8.0),
                            column_gap: Val::Px(8.0),
                            ..default()
                        },
                        background_color: Color::BLACK.into(),
                        ..default()
                    },
                    DialogUI,
                ))
                .with_children(|parent| {
                    for option in options {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Auto,
                                        height: Val::Auto,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: Color::BLACK.into(),
                                    ..default()
                                },
                                ButtonInteractionAction::<Text> {
                                    on_none: |_, _, _| {}, // custom logic
                                    ..default()
                                },
                                OptionUINode {
                                    node_title: option.node.to_string(),
                                    used: option.used,
                                },
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle {
                                        text: Text {
                                            sections: vec![TextSection {
                                                value: option.text.to_string(),
                                                style: TextStyle {
                                                    font: fonts.noir_font_regular.clone_weak(),
                                                    font_size: 20.0,
                                                    color: if option.used {
                                                        Color::GRAY
                                                    } else {
                                                        Color::WHITE
                                                    },
                                                },
                                            }],
                                            alignment: TextAlignment::Center,
                                            ..default()
                                        },
                                        style: Style {
                                            direction: Direction::RightToLeft,
                                            justify_content: JustifyContent::Start,
                                            align_content: AlignContent::Start,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    DialogUIText,
                                ));
                            });
                    }
                });
        });
}

fn build_dialog_ui(
    owning_entity: &mut EntityCommands,
    asset_server: &Res<AssetServer>,
    avatars: &mut ResMut<AvatarHandles>,
    fonts: &Res<Fonts>,
    speaker: &String,
    text: &String,
) {
    owning_entity
        .insert(CurrentDialogEvent)
        .insert((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(20.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    column_gap: Val::Px(20.0),
                    top: Val::Percent(80.0),
                    left: Val::Percent(15.0),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            DialogUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(160.0),
                        height: Val::Px(160.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: avatars.get_weak_or_add(speaker, asset_server).into(),
                    ..default()
                },
                DialogUIImage,
            ));
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Auto,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "".to_string(),
                                        style: TextStyle {
                                            font: fonts.noir_font_regular.clone_weak(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    }],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                style: Style {
                                    direction: Direction::RightToLeft,
                                    justify_content: JustifyContent::Start,
                                    align_content: AlignContent::Start,
                                    align_items: AlignItems::Center,
                                    flex_wrap: FlexWrap::Wrap,
                                    ..default()
                                },
                                ..default()
                            },
                            DialogUIText,
                        ))
                        .insert(create_type_writing_text(&text.to_string(), 0.05, None));
                });
        });
}

pub fn interact_with_dialog_text(
    mut button_query: Query<(&Interaction, &mut Children, &OptionUINode), Changed<Interaction>>,
    mut text_query: Query<&mut Text>,
    mut dialogs: ResMut<Dialogs>,
) {
    for (interaction, children, node) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                dialogs.runner.make_decision(&node.node_title);
            }
            Interaction::None => {
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(*child) {
                        let color = if node.used { Color::GRAY } else { Color::WHITE };
                        text.sections[0].style.color = color;
                    }
                }
            }
            _ => (),
        }
    }
}

pub fn start_dialog(
    mut commands: Commands,
    mut game_state: ResMut<NextState<InGameState>>,
    npc_dialog: Query<Entity, (Added<Clicked>, With<DialogableNPC>)>,
) {
    if !npc_dialog.is_empty() {
        commands
            .entity(npc_dialog.get_single().unwrap())
            .insert(NPCInDialog)
            .remove::<Clicked>();
        game_state.set(Dialog);
    }
}

pub fn load_dialog(
    mut commands: Commands,
    dialog_assets: Res<Assets<YarnSpinnerDialog>>,
    mut avatars_handle: ResMut<AvatarHandles>,
    asset_server: Res<AssetServer>,
    npc_dialog: Query<&DialogableNPC, Added<NPCInDialog>>,
    mut internal_dialog_state: ResMut<NextState<DialogInternalState>>,
    global_state: Res<GlobalState>,
) {
    let dialog_npc_config = npc_dialog.get_single().unwrap();
    let dialog = dialog_assets
        .get(dialog_npc_config.dialog_handle.clone_weak())
        .unwrap();
    let start_node = dialog_npc_config
        .node(global_state.get_value(dialog_npc_config.first_dialog_mark.as_str()));
    commands.remove_resource::<Dialogs>();
    commands.insert_resource(Dialogs {
        name: "test".to_string(),
        runner: DialogRunner::create_from_nodes(dialog.nodes.clone(), start_node),
        timer: Timer::from_seconds(1.0, Repeating),
    });
    avatars_handle.add_from_dialog(&dialog, &asset_server);
    internal_dialog_state.set(DialogInternalState::DialogAvatarLoading)
}

pub fn mouse_button_input(
    mut commands: Commands,
    mut global_state: ResMut<GlobalState>,
    buttons: Res<Input<MouseButton>>,
    mut dialogs: ResMut<Dialogs>,
    mut npc_dialog: Query<(Entity, &mut DialogableNPC), With<NPCInDialog>>,
    mut game_state: ResMut<NextState<InGameState>>,
    mut dialog_internal_state: ResMut<NextState<DialogInternalState>>,
) {
    let (entity, dialog_npc_config) = npc_dialog.get_single_mut().unwrap();
    if buttons.just_pressed(MouseButton::Left) {
        let event = dialogs.runner.next_event(&mut global_state, &mut commands);
        commands.spawn(DialogEventBundle {
            event: event.clone(),
            ownership: DialogEventOwnership::PARENT,
        });
        if let DialogEvent::End = event {
            game_state.set(InGameState::InGame);
            dialog_internal_state.set(DialogInternalState::NoDialog);
            commands.entity(entity).remove::<NPCInDialog>();
            dialogs
                .runner
                .reset_to(dialog_npc_config.reset_node.as_str());
        }
    }
}
