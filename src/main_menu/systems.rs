use bevy::app::AppExit;
use bevy::prelude::*;
use crate::game_state::GameState;
use crate::level_state::LevelState;
use crate::levels::systems::load_current_level;
use super::components::*;

pub fn initialize_main_menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((main_menu_bundle(), MainMenuUI))
        .with_children(|parent| {
            parent
                .spawn((main_menu_image(&asset_server), MainMenuImage))
                .with_children(|parent| {
                    parent
                        .spawn((new_game_button(), StartGameButton))
                        .with_children(|parent| {
                            parent.spawn((new_game_text(&asset_server), StartGameText));
                        });
                    parent
                        .spawn((quit_game_button(), QuitGameButton))
                        .with_children(|parent| {
                            parent.spawn((quit_game_text(&asset_server), QuitGameText));
                        });
                });
        });
}

pub fn despawn_main_menu_ui(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn new_game_button_click(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
    mut text_query: Query<&mut Text, With<StartGameText>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut level_state: ResMut<NextState<LevelState>>,
) {
    for interaction in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                level_state.set(LevelState::TrainPlatform);
                game_state.set(GameState::Intro);
            }
            Interaction::Hovered => {
                for mut text in text_query.iter_mut() {
                    text.sections[0].style.color = Color::RED;
                }
            }
            _ => {
                for mut text in text_query.iter_mut() {
                    text.sections[0].style.color = Color::WHITE;
                }
            }
        }
    }
}

pub fn quit_game_button_interaction(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<QuitGameButton>)>,
    mut text_query: Query<&mut Text, With<QuitGameText>>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                exit.send(AppExit);
            }
            Interaction::Hovered => {
                for mut text in text_query.iter_mut() {
                    text.sections[0].style.color = Color::RED;
                }
            }
            _ => {
                for mut text in text_query.iter_mut() {
                    text.sections[0].style.color = Color::WHITE;
                }
            }
        }
    }
}

fn main_menu_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(8.0),
            column_gap: Val::Px(8.0),
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    }
}

fn main_menu_image(asset_server: &Res<AssetServer>) -> ImageBundle {
    ImageBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        image: asset_server.load("images/main_menu/background.png").into(),
        ..default()
    }
}

fn new_game_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Auto,
            height: Val::Auto,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    }
}

fn new_game_text(asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        style: Style {
            width: Val::Auto,
            height: Val::Auto,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        text: Text {
            sections: vec![TextSection {
                value: "Nowa gra".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/Noir_regular.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            }],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    }
}

fn quit_game_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Auto,
            height: Val::Auto,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    }
}

fn quit_game_text(asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        style: Style {
            width: Val::Auto,
            height: Val::Auto,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        text: Text {
            sections: vec![TextSection {
                value: "Koniec gry".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/Noir_regular.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            }],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    }
}
