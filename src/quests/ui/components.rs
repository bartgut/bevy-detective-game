use bevy::prelude::*;
use crate::spawnable::components::Spawnable;

#[derive(Component)]
pub struct QuestLogUI;

#[derive(Component)]
pub struct QuestLogUISprite;

#[derive(Component)]
pub struct QuestLogListUI;

#[derive(Component)]
pub struct QuestLogListTitleUI;

#[derive(Component)]
pub struct QuestLogListActiveUI;

#[derive(Component)]
pub struct QuestDetails {
    pub title: String,
    pub description: String,
}

#[derive(Component)]
pub struct QuestLogDetailsUI;

#[derive(Component)]
pub struct QuestLogDetailsTitle;

#[derive(Component)]
pub struct QuestLogDetailsDescription;

impl Spawnable for QuestLogUI {
    fn spawn(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands
            .spawn((quest_log_ui_bundle(), QuestLogUI))
            .with_children(|parent| {
                parent
                    .spawn((quest_log_ui_image(asset_server), QuestLogUISprite))
                    .with_children(|parent| {
                        parent
                            .spawn((quest_log_ui_list_bundle(), QuestLogListUI))
                            .with_children(|parent| {
                                parent.spawn((
                                    quest_log_ui_details_title_bundle(),
                                    QuestLogListTitleUI,
                                ));
                                parent.spawn((
                                    quest_log_ui_details_description_bundle(),
                                    QuestLogListActiveUI,
                                ));
                            });
                        parent
                            .spawn((quest_log_ui_details_bundle(), QuestLogDetailsUI))
                            .with_children(|parent| {
                                parent.spawn((
                                    quest_log_ui_details_title_bundle(),
                                    QuestLogDetailsTitle,
                                ));
                                parent.spawn((
                                    quest_log_ui_details_description_bundle(),
                                    QuestLogDetailsDescription,
                                ));
                            });
                    });
            });
    }
}

fn quest_log_ui_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
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

fn quest_log_ui_list_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(45.0),
            height: Val::Percent(60.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            row_gap: Val::Percent(3.0),
            column_gap: Val::Percent(3.0),
            left: Val::Percent(5.0),
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    }
}

fn quest_log_ui_details_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(45.0),
            height: Val::Percent(60.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            right: Val::Percent(1.0),
            row_gap: Val::Percent(2.0),
            column_gap: Val::Percent(2.0),
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    }
}

fn quest_log_ui_details_title_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(20.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Percent(8.0),
            column_gap: Val::Percent(8.0),
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    }
}

fn quest_log_ui_details_description_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(80.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            row_gap: Val::Percent(8.0),
            column_gap: Val::Percent(8.0),
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    }
}

fn quest_log_ui_image(asset_server: &Res<AssetServer>) -> ImageBundle {
    ImageBundle {
        style: Style {
            width: Val::Percent(80.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            row_gap: Val::Percent(5.0),
            column_gap: Val::Percent(5.0),
            ..default()
        },
        image: asset_server.load("images/quest_log/quest_log.png").into(),
        ..default()
    }
}
