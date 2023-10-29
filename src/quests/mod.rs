use bevy::prelude::*;
use crate::game_state::GameState;
use crate::quests::systems::{activate_quests, complete_quests};
use crate::quests::ui::QuestLogUIPlugin;

pub struct QuestPlugin;

pub mod components;
pub mod systems;
pub mod ui;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuestLogUIPlugin)
            .add_systems(
            Update,
            (activate_quests, complete_quests).run_if(in_state(GameState::InGame)),
            )
            .add_systems(PreStartup, systems::init_quests);
    }
}
