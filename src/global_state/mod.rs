use bevy::prelude::*;
use crate::global_state::global_state::GlobalState;
use crate::global_state::systems::update_state_on_add;

pub mod global_state;
pub mod systems;

pub struct GlobalStatePlugin;

impl Plugin for GlobalStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalState::default())
            .add_systems(Update, update_state_on_add);
    }
}
