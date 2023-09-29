use bevy::audio::PlaybackMode::Despawn;
use bevy::prelude::*;
use crate::dialogs::dialog_runner::context::StateContext;
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
    updates: Query<(Entity, &AddCollectibleToInventory), Added<AddCollectibleToInventory>>,
) {
    for (entity, update) in updates.iter() {
        global_state.add_item(&update.0);
        commands
            .entity(entity)
            .remove::<AddCollectibleToInventory>();
    }
}

pub fn update_inventory_sound_effect_on_add(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    updates: Query<(Entity, &AddCollectibleToInventory), Added<AddCollectibleToInventory>>,
) {
    if !updates.is_empty() {
        commands.spawn(AudioBundle {
            source: asset_server.load("sound/items/item_received.ogg"),
            settings: PlaybackSettings {
                mode: Despawn,
                ..default()
            },
            ..default()
        });
    }
}
