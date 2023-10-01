use bevy::prelude::*;
use crate::clickable::items::onesideitem::OneSideItem;
use crate::clickable::items::systems::collectible_click;
use crate::clickable::items::twosideitem::TwoSideItem;
use crate::clickable::systems::{
    clickable_can_be_clicked, clickable_click, clickable_clicked, clickable_clicked_no_conditions,
    clickable_condition_check, clickable_first_click, gray_out_all, hover_entry, initialize_items,
    print_when_hovered_clickable, print_when_hovered_clickable_global, return_to_normal_colors,
};
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
        app.add_systems(OnEnter(GameState::InGame), initialize_items)
            .add_systems(OnEnter(InGameState::LookingAtItem), gray_out_all)
            .add_systems(OnExit(InGameState::LookingAtItem), return_to_normal_colors)
            .add_systems(
                Update,
                print_when_hovered_clickable.run_if(in_state(InGameState::InGame)),
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
            .add_systems(Update, collectible_click)
            .add_systems(Update, clickable_clicked::<OneSideItem>)
            .add_systems(Update, clickable_clicked::<TwoSideItem>)
            .add_systems(Update, clickable_clicked::<LevelTeleport>);
    }
}
