use bevy::prelude::*;

#[derive(Component)]
pub struct TypeWritingTextSettings {
    pub text: String,
    pub every: f32,
    pub cur_len: usize,
}

#[derive(Component)]
pub struct TypeWritingTextTimer(pub Timer);
