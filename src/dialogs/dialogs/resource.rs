use bevy::prelude::*;
use crate::dialogs::dialog_runner::runner::*;
use crate::parsing::yarnspinner::*;
use bevy::time::TimerMode::Repeating;
use crate::global_state::global_state::GlobalState;

#[derive(Resource)]
pub struct Dialogs {
    pub name: String,
    pub runner: DialogRunner<GlobalState>,
    pub timer: Timer,
}

impl Dialogs {
    pub fn load_from_file(file_name: &str, start_node: &str) -> Dialogs {
        let nodes = yarn_spinner_parsing::load_from_file(file_name);
        Dialogs {
            name: "test".to_string(),
            runner: DialogRunner::create_from_nodes(nodes, start_node),
            timer: Timer::from_seconds(1.0, Repeating),
        }
    }
}
