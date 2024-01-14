use bevy::prelude::*;
use bevy_yarnspinner::dialog_runner::context::StateContext;
use crate::global_state::global_state::ConditionFunc;
use crate::inventory::components::Inventory;

#[derive(Component)]
pub struct VisibilityCondition {
    pub conditions: Vec<ConditionFunc>,
}

impl VisibilityCondition {
    pub fn passed<T: StateContext + Inventory>(&self, state: &T) -> bool {
        self.conditions
            .iter()
            .all(|condition| condition.passed(state))
    }
}
