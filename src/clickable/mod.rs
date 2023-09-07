use bevy::prelude::*;
use crate::clickable::items::onesideitem::OneSideItem;
use crate::clickable::items::twosideitem::TwoSideItem;
use crate::clickable::systems::{
    clickable_can_be_clicked, clickable_click, clickable_first_click, gray_out_all, hover_entry,
    initialize_items, print_when_hovered_clickable, print_when_hovered_clickable_global,
    return_to_normal_colors,
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
            //.add_system(print_when_hovered_clickable.in_set(Update(InGameState::InGame)))
            .add_systems(
                Update,
                print_when_hovered_clickable_global.run_if(in_state(InGameState::Map)),
            )
            //.add_system(print_when_hovered_clickable_global.in_set(Update(InGameState::Map)))
            .add_systems(Update, clickable_can_be_clicked)
            .add_systems(Update, hover_entry::<LevelTeleport>)
            .add_systems(Update, clickable_first_click::<OneSideItem>)
            .add_systems(Update, clickable_first_click::<TwoSideItem>)
            .add_systems(Update, clickable_click::<OneSideItem>)
            .add_systems(Update, clickable_click::<TwoSideItem>)
            .add_systems(Update, clickable_first_click::<LevelTeleport>);
    }
}
