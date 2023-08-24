use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::level_state::LevelState;
use crate::spawnable::components::Spawnable;

#[derive(Resource)]
pub struct ItemResource {
    pub items: HashMap<LevelState, Vec<Box<dyn Spawnable + Send + Sync>>>,
}
