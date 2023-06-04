use bevy::prelude::*;
use crate::clickable::items::onesideitem::OneSideItem;
use crate::clickable::items::twosideitem::TwoSideItem;
use crate::clickable::systems::{
    clickable_can_be_clicked, clickable_click, clickable_first_click, gray_out_all,
    initialize_clickable, print_when_hovered_clickable, return_to_normal_colors,
};
use crate::game_state::GameState;
use crate::in_game_state::InGameState;

pub mod components;
pub mod constants;
pub mod items;
pub mod systems;

pub struct ClickablePlugin;

impl Plugin for ClickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(initialize_clickable::<OneSideItem>.in_schedule(OnEnter(GameState::InGame)))
            .add_system(initialize_clickable::<TwoSideItem>.in_schedule(OnEnter(GameState::InGame)))
            .add_system(gray_out_all.in_schedule(OnEnter(InGameState::LookingAtItem)))
            .add_system(return_to_normal_colors.in_schedule(OnExit(InGameState::LookingAtItem)))
            .add_system(print_when_hovered_clickable)
            .add_system(clickable_can_be_clicked)
            .add_system(clickable_first_click::<OneSideItem>)
            .add_system(clickable_first_click::<TwoSideItem>)
            .add_system(clickable_click::<OneSideItem>)
            .add_system(clickable_click::<TwoSideItem>);
    }
}
