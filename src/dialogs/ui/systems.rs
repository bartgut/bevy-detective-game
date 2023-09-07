use super::components::*;
use bevy::prelude::*;
use crate::dialogs::dialog_runner::components::DialogEvent;
use crate::dialogs::dialogs::resource::*;
use crate::in_game_state::InGameState;
use crate::npc::components::{DialogableNPC, HoveredOverNPC, NPCInDialog};
use crate::text::typewriting::systems::create_type_writing_text;

pub fn build_dialog_ui_from_event(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    event: &DialogEvent,
) {
    match event {
        DialogEvent::Dialog { speaker, text } => {
            build_dialog_ui(commands, asset_server, speaker, text);
        }
        DialogEvent::Options { speaker, options } => {
            build_options_ui(commands, asset_server, options)
        }
        _ => {}
    }
}

pub fn build_options_ui(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    options: &Vec<(String, String)>,
) {
    commands
        .spawn((
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
                    /*position: UiRect {
                        top: Val::Percent(80.0),
                        ..default()
                    },*/
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
                                OptionUINode {
                                    node_title: option.1.to_string(),
                                },
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle {
                                        text: Text {
                                            sections: vec![TextSection {
                                                value: option.0.to_string(),
                                                style: TextStyle {
                                                    font: asset_server
                                                        .load("fonts/Noir_regular.ttf"),
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

//image + text on the bottom of the screen
pub fn build_dialog_ui(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    speaker: &String,
    text: &String,
) -> Entity {
    commands
        .spawn((
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
                    /*position: UiRect {
                        top: Val::Percent(80.0),
                        left: Val::Percent(15.0),
                        ..default()
                    },*/
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
                    image: asset_server
                        .load(format!("images/avatars/{}.png", speaker))
                        .into(),
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
                                            font: asset_server.load("fonts/Noir_regular.ttf"),
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
        })
        .id()
}

pub fn interact_with_dialog_text(
    mut button_query: Query<(&Interaction, &mut Children, &OptionUINode), Changed<Interaction>>,
    mut text_query: Query<&mut Text>,
    mut dialogs: ResMut<Dialogs>,
) {
    for (interaction, children, node) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                dialogs.runner.make_decision(node.node_title.clone());
            }
            Interaction::Hovered => {
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(*child) {
                        text.sections[0].style.color = Color::RED
                    }
                }
            }
            Interaction::None => {
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(*child) {
                        text.sections[0].style.color = Color::WHITE
                    }
                }
            }
        }
    }
}

pub fn start_dialog(
    mut commands: Commands,
    mut game_state: ResMut<NextState<InGameState>>,
    npc_dialog: Query<Entity, With<HoveredOverNPC>>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if !npc_dialog.is_empty() {
            commands
                .entity(npc_dialog.get_single().unwrap())
                .insert(NPCInDialog);
            game_state.set(InGameState::Dialog);
        }
    }
}

pub fn load_dialog(
    mut commands: Commands,
    npc_dialog: Query<&DialogableNPC, With<HoveredOverNPC>>,
) {
    let dialog_npc_config = npc_dialog.get_single().unwrap();
    commands.remove_resource::<Dialogs>();
    commands.insert_resource(Dialogs::load_from_file(
        format!("assets/dialogs/{}.yarn", dialog_npc_config.dialog_file_name).as_str(),
        dialog_npc_config.node(),
    ));
}

pub fn mouse_button_input(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    buttons: Res<Input<MouseButton>>,
    mut dialogs: ResMut<Dialogs>,
    dialog_query: Query<Entity, With<DialogUI>>,
    option_query: Query<Entity, With<OptionUI>>,
    mut npc_dialog: Query<(Entity, &mut DialogableNPC), With<NPCInDialog>>,
    mut game_state: ResMut<NextState<InGameState>>,
) {
    let (entity, mut dialog_npc_config) = npc_dialog.get_single_mut().unwrap();
    if buttons.just_pressed(MouseButton::Left) {
        let event = dialogs.runner.next_event();
        match event {
            DialogEvent::Waiting => {}
            DialogEvent::End => {
                game_state.set(InGameState::InGame);
                if let Ok(dialog_entity) = dialog_query.get_single() {
                    commands.entity(dialog_entity).despawn_recursive();
                }
                if let Ok(option_entity) = option_query.get_single() {
                    commands.entity(option_entity).despawn_recursive();
                }
                dialog_npc_config.first_dialog = false;
                commands.entity(entity).remove::<NPCInDialog>();
                dialogs
                    .runner
                    .reset_to(dialog_npc_config.reset_node.as_str());
            }
            _ => {
                if let Ok(dialog_entity) = dialog_query.get_single() {
                    commands.entity(dialog_entity).despawn_recursive();
                }
                if let Ok(option_entity) = option_query.get_single() {
                    commands.entity(option_entity).despawn_recursive();
                }
                build_dialog_ui_from_event(&mut commands, asset_server, &event);
            }
        }
    }
}
