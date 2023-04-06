use super::components::*;
use bevy::prelude::*;
use crate::dialogs::dialog_runner::components::DialogEvent;
use crate::dialogs::dialogs::resource::*;

pub fn build_dialog_ui_from_event(mut commands: &mut Commands,
                                  asset_server: Res<AssetServer>,
                                  event: &DialogEvent) {
    match event {
        DialogEvent::Dialog { speaker, text } => {
            build_dialog_ui(commands, asset_server, speaker, text);
        }
        DialogEvent::Options { speaker, options} => {
            build_options_ui(commands, asset_server, speaker, options)
        }
        _ => {}
    }
}

pub fn build_options_ui(mut commands: &mut Commands,
                        asset_server: Res<AssetServer>,
                        speaker: &String,
                        options: &Vec<(String, String)>) {
    commands
        .spawn( (NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
                position: UiRect {
                    top: Val::Percent(80.0),
                    ..default()
                },
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        }, OptionUI))
        .with_children(|parent| {
            parent.spawn( (ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(160.0), Val::Px(160.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: asset_server.load("images/avatars/Player.png").into(),
                ..default()
            }, DialogUIImage));
            parent.spawn( (NodeBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            }, DialogUI))
                .with_children(|parent| {
                    for option in options {
                        parent.spawn( (ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::BLACK.into(),
                            ..default()
                        }, OptionUINode { node_title: option.1.to_string() }))
                            .with_children(|parent| {
                                parent.spawn( (TextBundle {
                                    text: Text {
                                        sections: vec![
                                            TextSection {
                                                value: option.0.to_string(),
                                                style: TextStyle {
                                                    font: asset_server.load("fonts/Noir_regular.ttf"),
                                                    font_size: 20.0,
                                                    color: Color::WHITE,
                                                },

                                            },
                                        ],
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
                                }, DialogUIText));
                            });
                    }
                });
        });
}

//image + text on the bottom of the screen
pub fn build_dialog_ui(mut commands: &mut Commands,
                       asset_server: Res<AssetServer>,
                       speaker: &String,
                       text: &String) {
    commands
        .spawn( (NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
                position: UiRect {
                    top: Val::Percent(80.0),
                    ..default()
                },
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        }, DialogUI))
        .with_children(|parent| {
            parent.spawn( (ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(160.0), Val::Px(160.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: asset_server.load(format!("images/avatars/{}.png", speaker)).into(),
                ..default()
            }, DialogUIImage));
            parent.spawn( ButtonBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            }).with_children(|parent| {
                parent.spawn( (TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: text.to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/Noir_regular.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },

                            },
                        ],
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
                }, DialogUIText));
            });
        });

}

pub fn interact_with_dialog_text(
    mut button_query: Query<(&Interaction, &mut BackgroundColor, &mut Children, &OptionUINode),
        Changed<Interaction>,>,
    mut text_query: Query<&mut Text>,
    mut dialogs: ResMut<Dialogs>) {
    for (interaction, mut background_color, mut children, mut node) in button_query.iter_mut() {
        //println!("{:?}", children);
        //println!("Any found?");
        match *interaction {
            Interaction::Clicked => {
                dialogs.runner.make_decision(node.node_title.clone());
            }
            Interaction::Hovered => {
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(*child) {
                        text.sections[0].style.color = Color::RED
                    }
                }
                //background_color.0 = Color::rgb(0.6, 0.6, 0.6);
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

pub fn mouse_button_input(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    buttons: Res<Input<MouseButton>>,
    mut dialogs: ResMut<Dialogs>,
    dialog_query: Query<Entity, With<DialogUI>>,
    option_query: Query<Entity, With<OptionUI>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let event = dialogs.runner.next_event();
        match event {
            DialogEvent::Waiting => {}
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