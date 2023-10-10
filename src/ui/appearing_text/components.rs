use bevy::prelude::*;
#[derive(Component)]
pub struct StartText;

#[derive(Component)]
pub struct AppearingTextSettings {
    not_visible_seconds: f32,
    appearing_seconds: f32,
    visible_seconds: f32,
    disappearing_seconds: f32,
}

impl Default for AppearingTextSettings {
    fn default() -> Self {
        AppearingTextSettings {
            not_visible_seconds: 1.0,
            appearing_seconds: 2.0,
            visible_seconds: 5.0,
            disappearing_seconds: 2.0,
        }
    }
}
/* Appearing text */
#[derive(Component)]
pub struct NotVisibleTimer(pub Timer);

#[derive(Component)]
pub struct AppearingTimer(pub Timer);

#[derive(Component)]
pub struct VisibleTimer(pub Timer);

#[derive(Component)]
pub struct DisappearingTimer(pub Timer);

#[derive(Bundle)]
pub struct AppearingTextBundle {
    pub not_visible_timer: NotVisibleTimer,
    pub appearing_timer: AppearingTimer,
    pub visible_timer: VisibleTimer,
    pub disappearing_timer: DisappearingTimer,
    pub start_state: AppearingTextState,
    pub text: TextBundle,
}

#[derive(Component)]
pub enum AppearingTextState {
    NOT_VISIBLE,
    APPEARING,
    VISIBLE,
    DISAPPEARING,
    FINISHED,
}
