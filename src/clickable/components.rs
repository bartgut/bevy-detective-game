use bevy::prelude::*;
use crate::dialogs::dialog_runner::context::StateContext;
use crate::global_state::global_state::GlobalState;
use crate::inventory::components::Inventory;

#[derive(Component)]
pub struct Clickable {
    pub level_initial_position: Vec3,
    pub required_distance: f32,
}

#[derive(Component)]
pub struct CanBeClicked;

#[derive(Component)]
pub struct HoveredOverClickable;

#[derive(Component)]
pub struct ClickConditionCheck;

#[derive(Component)]
pub struct Clicked;

pub enum ClickCondition {
    StateCondition(fn(&GlobalState) -> bool),
    InventoryCondition(fn(&GlobalState) -> bool),
}

#[derive(Component)]
pub struct ClickConditions {
    pub condition: Vec<ClickCondition>,
    pub failure: fn(&mut Commands, &Res<AssetServer>),
}
