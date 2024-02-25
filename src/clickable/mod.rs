use bevy::prelude::*;
use rive_bevy::GenericEvent;
use crate::clickable::items::onesideitem::OneSideItem;
use crate::clickable::items::rive_item::RiveItem;
use crate::clickable::items::systems::collectible_click;
use crate::clickable::items::twosideitem::TwoSideItem;
use crate::clickable::systems::{
    clickable_can_be_clicked, clickable_click, clickable_clicked, clickable_clicked_no_conditions,
    clickable_condition_check, clickable_first_click, cursor_change_on_hover, event_handler,
    gray_out_all, hover_entry, initialize_items, print_when_hovered_clickable,
    print_when_hovered_clickable_global, return_to_normal_colors,
};
use crate::game::items::flying_orb::flying_orb_prepare;
use crate::game::items::train_station_fog::train_station_fog_prepare;
use crate::game::items::train_station_rain::train_station_rain_prepare;
use crate::game_state::GameState;
use crate::in_game_state::InGameState;
use crate::levels::components::LevelTeleport;

pub mod components;
pub mod constants;
pub mod items;
pub mod systems;

pub struct ClickablePlugin;

impl Plugin for ClickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InLevelSpritesLoading), initialize_items)
            .add_systems(PreStartup, train_station_fog_prepare)
            .add_systems(PreStartup, train_station_rain_prepare)
            .add_systems(PreStartup, flying_orb_prepare)
            .add_systems(OnEnter(InGameState::LookingAtItem), gray_out_all)
            .add_systems(OnExit(InGameState::LookingAtItem), return_to_normal_colors)
            .add_systems(
                Update,
                print_when_hovered_clickable.run_if(in_state(InGameState::InGame)),
            )
            .add_systems(
                Update,
                print_when_hovered_clickable.run_if(in_state(InGameState::Dialog)),
            )
            .add_systems(
                Update,
                print_when_hovered_clickable_global.run_if(in_state(InGameState::Map)),
            )
            .add_systems(Update, clickable_can_be_clicked)
            .add_systems(Update, hover_entry::<LevelTeleport>)
            .add_systems(Update, clickable_first_click)
            .add_systems(Update, clickable_condition_check)
            .add_systems(Update, clickable_clicked_no_conditions)
            .add_systems(Update, clickable_click::<OneSideItem>)
            .add_systems(Update, clickable_click::<TwoSideItem>)
            .add_systems(Update, clickable_clicked::<OneSideItem>)
            .add_systems(Update, clickable_clicked::<TwoSideItem>)
            .add_systems(Update, clickable_click::<RiveItem>)
            .add_systems(Update, clickable_clicked::<RiveItem>)
            .add_systems(Update, clickable_clicked::<LevelTeleport>)
            .add_systems(Update, event_handler::<GenericEvent>)
            .add_systems(Update, cursor_change_on_hover)
            .add_systems(PostUpdate, collectible_click);
    }
}
