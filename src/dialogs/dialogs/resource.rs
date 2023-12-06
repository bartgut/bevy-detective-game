use bevy::prelude::*;
use crate::dialogs::dialog_runner::runner::*;
use crate::global_state::global_state::GlobalState;

#[derive(Resource)]
pub struct Dialogs {
    pub name: String,
    pub runner: DialogRunner<GlobalState>,
    pub timer: Timer,
}
