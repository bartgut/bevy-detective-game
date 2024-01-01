use bevy::prelude::*;
use crate::global_state::global_state::{GlobalState, ConditionFunc};

#[derive(Component)]
pub struct Clickable {
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

#[derive(Component)]
pub struct ClickConditions {
    pub condition: Vec<ConditionFunc>,
    pub failure: fn(&mut Commands, &Res<AssetServer>),
}

impl ClickConditions {
    pub fn passed(&self, state: &GlobalState) -> bool {
        self.condition
            .iter()
            .map(|condition| condition.passed(state))
            .all(|x| x)
    }
}
