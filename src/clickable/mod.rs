use bevy::prelude::*;
use crate::clickable::items::onesideitem::OneSideItem;
use crate::clickable::items::twosideitem::TwoSideItem;
use crate::clickable::systems::{
    clickable_can_be_clicked, clickable_click, clickable_first_click, initialize_clickable,
    print_when_hovered_clickable,
};
use crate::game_state::GameState;

pub mod components;
pub mod constants;
pub mod items;
pub mod systems;

pub struct ClickablePlugin;

impl Plugin for ClickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(initialize_clickable::<OneSideItem>.in_schedule(OnEnter(GameState::InGame)))
            .add_system(initialize_clickable::<TwoSideItem>.in_schedule(OnEnter(GameState::InGame)))
            .add_system(print_when_hovered_clickable)
            .add_system(clickable_can_be_clicked)
            .add_system(clickable_first_click::<OneSideItem>)
            .add_system(clickable_first_click::<TwoSideItem>)
            .add_system(clickable_click::<OneSideItem>)
            .add_system(clickable_click::<TwoSideItem>);
    }
}
