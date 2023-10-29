use bevy::prelude::*;
use crate::assets::fonts::Fonts;
use crate::quests::components::{Quest, QuestStatus};
use crate::quests::ui::components::{
    QuestDetails, QuestLogDetailsDescription, QuestLogDetailsTitle, QuestLogListActiveUI,
    QuestLogListTitleUI, QuestLogListUI, QuestLogUI,
};
use crate::spawnable::components::Spawnable;

pub fn on_enter(mut commands: Commands, asset_server: Res<AssetServer>, quests: Query<&Quest>) {
    QuestLogUI.spawn(&mut commands, &asset_server);
}

pub fn quest_buttons(
    mut commands: Commands,
    font: Res<Fonts>,
    quests: Query<&Quest>,
    quest_log_ui: Query<Entity, Added<QuestLogListActiveUI>>,
    quest_log_title: Query<Entity, Added<QuestLogListTitleUI>>,
) {
    let (active_quests, completed_quests): (Vec<_>, Vec<_>) = quests
        .iter()
        .filter(|quest| {
            quest.status == QuestStatus::Active || quest.status == QuestStatus::Complete
        })
        .partition(|quest| quest.status == QuestStatus::Active);

    for title in quest_log_title.iter() {
        commands.entity(title).with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "AKTYWNE ZADANIA".to_string(),
                        style: TextStyle {
                            font: font.noir_font_bold.clone_weak(),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    }],
                    ..default()
                },
                ..default()
            });
        });
    }

    for quest_log_ui in quest_log_ui.iter() {
        commands.entity(quest_log_ui).with_children(|parent| {
            for active_quest in active_quests.iter() {
                println!("Active quest: {:?}", active_quest.short_description);
                parent
                    .spawn(active_quest_button(active_quest))
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: active_quest.short_description.clone(),
                                    style: TextStyle {
                                        font: font.noir_font_regular.clone_weak(),
                                        font_size: 20.0,
                                        color: Color::BLACK,
                                    },
                                }],
                                ..default()
                            },
                            ..default()
                        });
                    });
            }
        });
    }
}

pub fn quest_buttons_interaction(
    mut commands: Commands,
    mut button_query: Query<(&Interaction, &QuestDetails), Changed<Interaction>>,
    mut details_title: Query<Entity, With<QuestLogDetailsTitle>>,
    mut details_description: Query<Entity, With<QuestLogDetailsDescription>>,
    font: Res<Fonts>,
) {
    for (interaction, quest_details) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                let title = details_title.single_mut();
                let description = details_description.single_mut();
                commands
                    .entity(title)
                    .despawn_descendants()
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: quest_details.title.clone(),
                                    style: TextStyle {
                                        font: font.noir_font_bold.clone_weak(),
                                        font_size: 30.0,
                                        color: Color::BLACK,
                                    },
                                }],
                                ..default()
                            },
                            ..default()
                        });
                    });
                commands
                    .entity(description)
                    .despawn_descendants()
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: quest_details.description.clone(),
                                    style: TextStyle {
                                        font: font.noir_font_regular.clone_weak(),
                                        font_size: 15.0,
                                        color: Color::DARK_GRAY,
                                    },
                                }],
                                ..default()
                            },
                            ..default()
                        });
                    });
            }
            _ => (),
        }
    }
}

fn active_quest_button(quest: &Quest) -> (ButtonBundle, QuestDetails) {
    (
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
        },
        QuestDetails {
            title: quest.short_description.clone(),
            description: quest.long_description.clone(),
        },
    )
}

pub fn on_exit(mut commands: Commands, quest_log: Query<Entity, With<QuestLogUI>>) {
    for entity in quest_log.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
