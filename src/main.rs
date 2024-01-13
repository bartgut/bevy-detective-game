pub mod animation;
pub mod clickable;
pub mod comics;
pub mod comics_state;
pub mod dialogs;
pub mod event_journal;
pub mod game;
pub mod game_items;
pub mod game_levels;
pub mod game_npc;
pub mod game_state;
pub mod global_state;
pub mod in_game_state;
pub mod intro;
pub mod intro_state;
pub mod inventory;
pub mod keyboard;
pub mod level_state;
pub mod levels;
pub mod main_menu;
pub mod movement;
pub mod npc;
pub mod player;
pub mod quests;
pub mod sound;
pub mod spawnable;
pub mod text;
pub mod ui;

use bevy::prelude::*;
use bevy::utils::tracing::callsite::register;
use bevy_yarnspinner::plugin::yarn_spinner_plugin::YarnSpinnerPlugin;
use rive_bevy::RivePlugin;
use crate::animation::SpriteAnimationPlugin;
use crate::assets::asset_loading_monitor::AssetLoadingStateChangeExt;
use crate::assets::AssetsPlugin;
use crate::clickable::ClickablePlugin;
use crate::clickable::items::resource::ItemResource;
use crate::comics::ComicsPlugin;
use crate::comics::rive::components::RiveComicsSink;
use crate::comics::rive::RiveComicsPlugin;
use crate::comics_state::{ComicsState, MultiPageComicsState};
use crate::conditional_visibility::VisibilityConditionalPlugin;
use crate::dialogs::DialogsPlugin;
use crate::event_journal::EventJournalPlugin;
use crate::game::world_map::WorldMapPlugin;
use crate::game::yarn_functions::yarn_commands_register;
use crate::game_items::items_map;
use crate::game_levels::level_map;
use crate::game_npc::npc_map;
use crate::game_state::GameState;
use crate::global_state::global_state::GlobalState;
use crate::global_state::GlobalStatePlugin;
use crate::in_game_state::InGameState;
use crate::intro::IntroPlugin;
use crate::intro_state::IntroState;
use crate::intro_state::IntroState::Comics1Loading;
use crate::keyboard::KeyboardPlugin;
use crate::level_state::LevelState;
use crate::levels::LevelPlugin;
use crate::levels::resource::LevelsResource;
use crate::main_menu::MainMenuPlugin;
use crate::movement::MovementPlugin;
use crate::npc::NpcPlugin;
use crate::npc::resource::NPCResource;
use crate::player::PlayerPlugin;
use crate::quests::QuestPlugin;
use crate::sound::SoundPlugin;
use crate::text::TypeWritingTextPlugin;
use crate::ui::UIUtilsPlugin;

pub mod assets;
mod conditional_visibility;
extern crate pest;
#[macro_use]
extern crate pest_derive;

////////////////////////////////////////////////////

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RivePlugin)
        .add_plugins(YarnSpinnerPlugin)
        .add_systems(PreStartup, camera_setup)
        .add_systems(PreStartup, yarn_setup)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(LevelsResource {
            levels: level_map(),
        })
        .insert_resource(NPCResource { npcs: npc_map() })
        .insert_resource(ItemResource { items: items_map() })
        .insert_resource(GlobalState::default())
        .add_state::<IntroState>()
        .add_state::<GameState>()
        .add_state::<InGameState>()
        .add_state::<LevelState>()
        .add_state::<ComicsState>()
        .add_state::<MultiPageComicsState>()
        .add_plugins(DialogsPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(IntroPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(NpcPlugin)
        .add_plugins(TypeWritingTextPlugin)
        .add_plugins(ClickablePlugin)
        .add_plugins(ComicsPlugin)
        .add_plugins(RiveComicsPlugin)
        .add_plugins(UIUtilsPlugin)
        .add_plugins(SpriteAnimationPlugin)
        .add_plugins(WorldMapPlugin)
        .add_plugins(SoundPlugin)
        .add_plugins(KeyboardPlugin)
        .add_plugins(GlobalStatePlugin)
        .add_plugins(EventJournalPlugin)
        .add_plugins(QuestPlugin)
        .add_plugins(AssetsPlugin)
        .add_plugins(VisibilityConditionalPlugin)
        .run()
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn yarn_setup(mut commands: Commands) {
    yarn_commands_register();
}
