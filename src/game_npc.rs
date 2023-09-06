use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::game::npc::librarian::components::Librarian;
use crate::game::npc::railwayman::components::Railwayman;
use crate::level_state::LevelState;
use crate::npc::components::SpawnableNPC;

// ALL NPCS FOR ALL LEVELS

pub fn npc_map() -> HashMap<LevelState, Vec<Box<dyn SpawnableNPC + Send + Sync>>> {
    let mut map = HashMap::<LevelState, Vec<Box<dyn SpawnableNPC + Send + Sync>>>::new();
    map.entry(LevelState::TrainPlatform)
        .or_insert(vec![Box::new(Railwayman), Box::new(Librarian)]);
    map
}
