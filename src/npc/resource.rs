use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::level_state::LevelState;
use crate::npc::components::SpawnableNPC;

#[derive(Resource)]
pub struct NPCResource {
    pub npcs: HashMap<LevelState, Vec<Box<dyn SpawnableNPC + Send + Sync>>>,
}
