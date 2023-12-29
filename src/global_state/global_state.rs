use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_yarnspinner::dialog_runner::context::StateContext;
use crate::clickable::items::components::Collectible;
use crate::global_state::global_state::JournalEventMessage::AddedToInventory;
use crate::event_journal::components::{ComponentToEvent, JournalEventMessage};
use crate::inventory::components::Inventory;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct UpdateGlobalState(pub String, pub bool);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct AddCollectibleToInventory(pub Collectible);

impl ComponentToEvent for AddCollectibleToInventory {
    fn to_event(&self) -> JournalEventMessage {
        AddedToInventory {
            0: format!("Dodano przedmiot {}", self.0.name),
        }
    }
}

#[derive(Resource)]
pub struct GlobalState {
    flags: HashMap<String, bool>,
    inventory: Vec<Collectible>,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            flags: HashMap::default(),
            inventory: Vec::default(),
        }
    }
}

impl StateContext for GlobalState {
    fn get_value(&self, key: &str) -> Option<&bool> {
        self.flags.get(key)
    }

    fn set_value(&mut self, key: &str, value: &bool) {
        self.flags.insert(key.to_string(), *value);
    }
}

impl Inventory for GlobalState {
    fn add_item(&mut self, collectible: &Collectible) {
        self.inventory.push(collectible.clone());
    }

    fn get_all_items(&self) -> Vec<Collectible> {
        self.inventory.clone()
    }
}
