use bevy::prelude::*;
use super::components::*;
use super::bundles::*;

pub fn create_type_writing_text(text_to_show: &String, period: f32) -> TypeWritingTextBundle {
    return TypeWritingTextBundle {
        timer: TypeWritingTextTimer(Timer::from_seconds(period, TimerMode::Repeating)),
        settings: TypeWritingTextSettings {
            text: text_to_show.clone(),
            every: period,
            cur_len: 0,
        },
    };
}

pub fn type_writing_len_update(
    mut type_writing_query: Query<(&mut TypeWritingTextSettings, &mut TypeWritingTextTimer)>,
    time: Res<Time>,
) {
    for (mut setting, mut timer) in type_writing_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            if setting.cur_len != setting.text.len() {
                setting.cur_len += 1;
            }
        }
    }
}

pub fn type_writing_text_update(mut query: Query<(&mut TypeWritingTextSettings, &mut Text)>) {
    for (mut settings, mut text) in query.iter_mut() {
        text.sections[0].value = settings.text.chars().take(settings.cur_len).collect()
    }
}
