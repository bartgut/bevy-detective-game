use bevy::audio::PlaybackMode::Despawn;
use bevy::prelude::*;
use bevy::ui::PositionType::Absolute;
use crate::event_journal::components::{JournalEventMessage, JournalEventUI};
use crate::ui::appearing_text::components::{
    AppearingTextBundle, AppearingTimer, DisappearingTimer, NotVisibleTimer, VisibleTimer,
};
use crate::ui::appearing_text::components::AppearingTextState::NOT_VISIBLE;

pub fn ui_setup(mut commands: Commands) {
    commands
        .spawn(
            (NodeBundle {
                style: Style {
                    position_type: Absolute,
                    top: Val::Percent(20.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::BLACK.with_a(0.0).into(),
                ..default()
            }),
        )
        .insert(JournalEventUI);
}
pub fn on_event_received(
    mut commands: Commands,
    mut ui_node: Query<Entity, With<JournalEventUI>>,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<JournalEventMessage>,
) {
    for event in event_reader.iter() {
        match event {
            JournalEventMessage::AddedToInventory(msg) => {
                event_ui_text(&mut commands, &asset_server, &ui_node.single(), &msg);
                audio_setup(&mut commands, &asset_server, "sound/items/item_received.ogg")
            }
            JournalEventMessage::NewQuest(msg) => {
                event_ui_text(&mut commands, &asset_server, &ui_node.single(), &msg)
            }
            JournalEventMessage::QuestCompleted(msg) => {
                event_ui_text(&mut commands, &asset_server, &ui_node.single(), &msg)
            }
        }
    }
}

fn event_ui_text(commands: &mut Commands, asset_server: &Res<AssetServer>, ui_node: &Entity, text: &str) {
        commands.entity(*ui_node).with_children(|parent| {
            parent.spawn(AppearingTextBundle {
                not_visible_timer: NotVisibleTimer(Timer::from_seconds(1.0, TimerMode::Once)),
                appearing_timer: AppearingTimer(Timer::from_seconds(0.0, TimerMode::Once)),
                visible_timer: VisibleTimer(Timer::from_seconds(2.0, TimerMode::Once)),
                disappearing_timer: DisappearingTimer(Timer::from_seconds(1.0, TimerMode::Once)),
                start_state: NOT_VISIBLE,
                text: TextBundle::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/Noir_regular.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE.with_a(0.0),
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style { ..default() }),
            });
        });
}

fn audio_setup(commands: &mut Commands, asset_server: &Res<AssetServer>, audio: &str) {
        commands.spawn(AudioBundle {
            source: asset_server.load(audio),
            settings: PlaybackSettings {
                mode: Despawn,
                ..default()
            },
            ..default()
        });
}

