use bevy::prelude::*;
use crate::game_state::GameState;
use crate::quests::loader::asset::QuestBundleLoader;
use crate::quests::loader::format::{QuestBundle, QuestEventMessage};
use crate::quests::systems::{activate_quest_v2, complete_quest_v2, to_journal_event};
use crate::quests::ui::QuestLogUIPlugin;

pub struct QuestPlugin;

pub mod components;
pub mod loader;
pub mod systems;
pub mod ui;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<QuestBundle>()
            .init_asset_loader::<QuestBundleLoader>()
            .add_event::<QuestEventMessage>()
            .add_plugins(QuestLogUIPlugin)
            .add_systems(
                Update,
                (activate_quest_v2, complete_quest_v2, to_journal_event)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(PreStartup, systems::init_quests);
    }
}
