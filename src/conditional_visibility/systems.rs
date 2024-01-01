use bevy::prelude::*;
use bevy_yarnspinner::dialog_runner::context::StateContext;
use crate::conditional_visibility::components::VisibilityCondition;
use crate::inventory::components::Inventory;

pub fn visibility_on_added<T: StateContext + Inventory + Resource>(
    state_context: Res<T>,
    mut query: Query<(&VisibilityCondition, &mut Visibility), Added<Visibility>>,
) {
    query.for_each_mut(|(conditions, mut visibility)| {
        pass_check(conditions, visibility.as_mut(), state_context.as_ref())
    })
}

pub fn visibility_on_state_change<T: StateContext + Inventory + Resource>(
    state_context: Res<T>,
    mut query: Query<(&VisibilityCondition, &mut Visibility)>,
) {
    if state_context.is_changed() {
        query.for_each_mut(|(condition, mut visibility)| {
            pass_check(condition, visibility.as_mut(), state_context.as_ref())
        })
    }
}

fn pass_check<T: StateContext + Inventory>(
    condition: &VisibilityCondition,
    curr_visibility: &mut Visibility,
    state: &T,
) {
    if condition.passed(state) {
        *curr_visibility = Visibility::Visible
    } else {
        *curr_visibility = Visibility::Hidden
    }
}
