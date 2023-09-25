use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::dialogs::dialog_runner::context::StateContext;

#[derive(Component)]
pub struct UpdateGlobalState(pub String, pub bool);

#[derive(Resource)]
pub struct GlobalState {
    flags: HashMap<String, bool>,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            flags: HashMap::default(),
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
