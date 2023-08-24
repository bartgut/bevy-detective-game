use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::level_state::LevelState;
use crate::npc::components::SpawnableNPC;
use crate::npc::librarian::components::Librarian;
use crate::npc::railwayman::components::Railwayman;

// ALL NPCS FOR ALL LEVELS

pub fn npc_map() -> HashMap<LevelState, Vec<Box<dyn SpawnableNPC + Send + Sync>>> {
    let mut map = HashMap::<LevelState, Vec<Box<dyn SpawnableNPC + Send + Sync>>>::new();
    map.entry(LevelState::TrainPlatform)
        .or_insert(vec![])
        .push(Box::new(Railwayman));
    map.entry(LevelState::TrainPlatform)
        .or_insert(vec![])
        .push(Box::new(Librarian));
    map
}
