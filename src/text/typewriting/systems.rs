use bevy::prelude::*;
use super::components::*;
use super::bundles::*;
use rand::prelude::*;
use crate::sound::systems::typewriting_sound;

pub fn create_type_writing_text(
    text_to_show: &String,
    period: f32,
    start_len: Option<usize>,
) -> TypeWritingTextBundle {
    return TypeWritingTextBundle {
        timer: TypeWritingTextTimer(Timer::from_seconds(period, TimerMode::Once)),
        settings: TypeWritingTextSettings {
            text: text_to_show.clone(),
            every: period,
            randomizing: 0.05,
            cur_len: start_len.unwrap_or(0),
        },
    };
}

pub fn type_writing_len_update(
    mut commands: Commands,
    mut type_writing_query: Query<(
        Entity,
        &mut TypeWritingTextSettings,
        &mut TypeWritingTextTimer,
    )>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (entity, mut setting, mut timer) in type_writing_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            if setting.cur_len != setting.text.len() {
                setting.cur_len += 1;
                typewriting_sound(
                    &asset_server,
                    &audio,
                    setting.text.chars().nth(setting.cur_len - 1),
                );
                commands
                    .entity(entity)
                    .insert(TypeWritingTextTimer(Timer::from_seconds(
                        setting.every
                            + thread_rng().gen_range(-setting.randomizing..setting.randomizing),
                        TimerMode::Once,
                    )));
            } else {
                commands.entity(entity).remove::<TypeWritingTextTimer>();
                commands.entity(entity).insert(TypeWritingFinished);
            }
        }
    }
}

pub fn type_writing_text_update(mut query: Query<(&mut TypeWritingTextSettings, &mut Text)>) {
    for (mut settings, mut text) in query.iter_mut() {
        text.sections[0].value = settings.text.chars().take(settings.cur_len).collect()
    }
}

/// This system is used to update the text with pauses
pub fn create_type_writing_text_with_pauses(
    text: Vec<TextWithPause>,
) -> (TypeWritingWithPausesBundle, TypeWritingTextBundle) {
    return (
        TypeWritingWithPausesBundle {
            settings: TypeWritingWithPausesSettings {
                text: text,
                full_text: "".to_string(),
                cur_text: 0,
            },
        },
        create_type_writing_text(&"".to_string(), 0.05, None),
    );
}

pub fn type_writing_with_pauses_update(
    mut commands: Commands,
    mut type_writing_query: Query<(Entity, &mut TypeWritingWithPausesSettings)>,
    mut previous_type_writing_finished: Query<Entity, With<TypeWritingFinished>>,
    mut previous_type_writing_paused: Query<
        (Entity, &mut TypeWritingPauseTimer),
        With<TypeWritingPaused>,
    >,
    time: Res<Time>,
) {
    for (entity, mut settings) in type_writing_query.iter_mut() {
        if settings.cur_text == settings.text.len() {
            commands.entity(entity).insert(TypeWritingWithPauseFinished);
        } else {
            for entity in previous_type_writing_finished.iter_mut() {
                commands.entity(entity).remove::<TypeWritingFinished>();
                commands
                    .entity(entity)
                    .insert(TypeWritingPauseTimer(Timer::from_seconds(
                        settings.text[settings.cur_text].pause,
                        TimerMode::Once,
                    )));
                commands.entity(entity).insert(TypeWritingPaused);
            }
            for (entity, mut timer) in previous_type_writing_paused.iter_mut() {
                if timer.0.tick(time.delta()).just_finished() {
                    //// Load next text
                    commands.entity(entity).remove::<TypeWritingPaused>();
                    let current_text_len = settings.full_text.len();
                    let nextText = settings.text[settings.cur_text].textSettings.text.clone();
                    let mut full_text = &mut settings.full_text;
                    full_text.push_str(nextText.as_str());
                    commands.entity(entity).insert(create_type_writing_text(
                        &settings.full_text,
                        settings.text[settings.cur_text].textSettings.every,
                        Some(current_text_len),
                    ));
                    settings.cur_text += 1;
                    /////
                }
            }
        }
    }
}
