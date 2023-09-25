use bevy::prelude::*;
use crate::dialogs::dialog_runner::context::StateContext;
use crate::global_state::global_state::{GlobalState, UpdateGlobalState};

pub fn update_state_on_add(
    mut commands: Commands,
    mut global_state: ResMut<GlobalState>,
    updates: Query<(Entity, &UpdateGlobalState), Added<UpdateGlobalState>>,
) {
    for (entity, update) in updates.iter() {
        global_state.set_value(&update.0, &update.1);
        commands.entity(entity).remove::<UpdateGlobalState>();
    }
}
