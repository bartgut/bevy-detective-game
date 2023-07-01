use bevy::prelude::*;
use crate::text::typewriting::components::TextWithPause;
use crate::text::typewriting::systems::{
    create_type_writing_text, create_type_writing_text_with_pauses,
};
use crate::ui::components::FullScreenText;

pub fn full_screen_text(
    mut command: &mut Commands,
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
