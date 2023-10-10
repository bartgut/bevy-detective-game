use bevy::prelude::*;
use crate::ui::appearing_text::components::{
    AppearingTextState, AppearingTimer, DisappearingTimer, NotVisibleTimer, VisibleTimer,
};
use crate::ui::appearing_text::components::AppearingTextState::{
    APPEARING, DISAPPEARING, FINISHED, NOT_VISIBLE, VISIBLE,
};
macro_rules! state_change_macro {
    ($func_name:ident, $timer:ty, $start_state:pat, $end_state:expr, $additional_logic:expr) => {
        pub fn $func_name(
            time: Res<Time>,
            mut query: Query<(&mut Text, &mut AppearingTextState, &mut $timer)>,
        ) {
            for (mut text, mut state, mut timer) in query.iter_mut() {
                match *state {
                    $start_state => {
                        if timer.0.tick(time.delta()).just_finished() {
                            *state = $end_state;
                        }
                        $additional_logic(&mut text, &timer);
                    }
                    _ => (),
                }
            }
        }
    };
}

state_change_macro!(
    not_visible_setup,
    NotVisibleTimer,
    NOT_VISIBLE,
    APPEARING,
    |text, timer| ()
);

state_change_macro!(
    appearing_setup,
    AppearingTimer,
    APPEARING,
    VISIBLE,
    |text: &mut Mut<Text>, timer: &Mut<AppearingTimer>| {
        text.sections[0].style.color =
            Color::WHITE.with_a(1.0 / timer.0.duration().as_secs_f32() * timer.0.elapsed_secs())
    }
);

state_change_macro!(
    visible_setup,
    VisibleTimer,
    VISIBLE,
    DISAPPEARING,
    |text, timer| ()
);

state_change_macro!(
    disappearing_setup,
    DisappearingTimer,
    DISAPPEARING,
    FINISHED,
    |text: &mut Mut<Text>, timer: &Mut<DisappearingTimer>| text.sections[0].style.color =
        Color::WHITE.with_a(1.0 - 1.0 / timer.0.duration().as_secs_f32() * timer.0.elapsed_secs())
);

pub fn cleanup_setup(mut commands: Commands, query: Query<(Entity, &mut AppearingTextState)>) {
    for (entity, state) in query.iter() {
        match *state {
            FINISHED => {
                commands.entity(entity).despawn_recursive();
            }
            _ => (),
        }
    }
}
