use bevy::prelude::*;
use bevy_yarnspinner::dialog_runner::runner::DialogRunner;
use crate::global_state::global_state::GlobalState;

#[derive(Resource)]
pub struct Dialogs {
    pub name: String,
    pub runner: DialogRunner<GlobalState>,
    pub timer: Timer,
}
