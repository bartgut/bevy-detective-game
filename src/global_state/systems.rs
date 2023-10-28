use bevy::audio::PlaybackMode::Despawn;
use bevy::prelude::*;
use crate::dialogs::dialog_runner::context::StateContext;
use crate::event_journal::components::{ComponentToEvent, JournalEventMessage};
use crate::global_state::global_state::{AddCollectibleToInventory, UpdateGlobalState};
use crate::inventory::components::Inventory;

pub fn update_state_on_add<T: StateContext + Resource>(
    mut commands: Commands,
    mut global_state: ResMut<T>,
    updates: Query<(Entity, &UpdateGlobalState), Added<UpdateGlobalState>>,
) {
    for (entity, update) in updates.iter() {
        global_state.set_value(&update.0, &update.1);
        commands.entity(entity).remove::<UpdateGlobalState>();
    }
}

pub fn update_inventory_on_add<T: Inventory + Resource>(
    mut commands: Commands,
    mut global_state: ResMut<T>,
    mut journal_event: EventWriter<JournalEventMessage>,
    updates: Query<(Entity, &AddCollectibleToInventory), Added<AddCollectibleToInventory>>,
) {
    for (entity, update) in updates.iter() {
        global_state.add_item(&update.0);
        journal_event.send(update.to_event());
        commands
            .entity(entity)
            .remove::<AddCollectibleToInventory>();
    }
}
