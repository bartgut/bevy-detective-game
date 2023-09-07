use bevy::prelude::*;
use crate::text::typewriting::components::TextWithPause;
use crate::text::typewriting::systems::{create_type_writing_text_with_pauses};
use crate::ui::components::{FullScreenText, InvisibleToVisibleTransition};

pub fn full_screen_text(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    text: Vec<TextWithPause>,
) {
    command
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
            FullScreenText,
        ))
        .insert(create_type_writing_text_with_pauses(text));
}

pub fn invisible_to_visible_transition(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut BackgroundColor,
        &mut InvisibleToVisibleTransition,
    )>,
    timer: Res<Time>,
) {
    for (entity, mut background_color, mut transition_settings) in query.iter_mut() {
        if transition_settings.0.tick(timer.delta()).just_finished() {
            background_color.0.set_a(1.0);
            commands
                .entity(entity)
                .remove::<InvisibleToVisibleTransition>();
        } else {
            background_color.0.set_a(
                transition_settings.0.elapsed_secs()
                    / transition_settings.0.duration().as_secs_f32(),
            );
        }
    }
}
