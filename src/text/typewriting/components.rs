use bevy::prelude::*;

#[derive(Component)]
pub struct TypeWritingTextSettings {
    pub text: String,
    pub every: f32,
    pub randomizing: f32,
    pub cur_len: usize,
}

#[derive(Component)]
pub struct TypeWritingTextTimer(pub Timer);

#[derive(Component)]
pub struct TypeWritingPauseTimer(pub Timer);

#[derive(Component)]
pub struct TypeWritingFinished;

#[derive(Component)]
pub struct TypeWritingPaused;

#[derive(Component)]
pub struct TypeWritingWithPauseFinished;

/// Typewriting with pauses

pub struct TextWithPause {
    pub text_settings: TypeWritingTextSettings,
    pub pause: f32,
}

#[derive(Component)]
pub struct TypeWritingWithPausesSettings {
    pub text: Vec<TextWithPause>,
    pub full_text: String,
    pub cur_text: usize,
}
