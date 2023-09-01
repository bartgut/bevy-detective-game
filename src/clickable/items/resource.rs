use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::level_state::LevelState;
use crate::spawnable::components::SpawnableChild;

#[derive(Resource)]
pub struct ItemResource {
    pub items: HashMap<LevelState, Vec<Box<dyn SpawnableChild + Send + Sync>>>,
}
