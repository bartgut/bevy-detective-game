use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::level_state::LevelState;
use crate::levels::components::LevelDescription;
use crate::npc::components::{DialogableNPC, NPC};

#[derive(Resource)]
pub struct NPCResource {
    pub npcs: HashMap<LevelState, Vec<(NPC, DialogableNPC)>>,
}
