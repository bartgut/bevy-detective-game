use super::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct TypeWritingTextBundle {
    pub settings: TypeWritingTextSettings,
    pub timer: TypeWritingTextTimer,
}

#[derive(Bundle)]
pub struct TypeWritingWithPausesBundle {
    pub settings: TypeWritingWithPausesSettings,
}
