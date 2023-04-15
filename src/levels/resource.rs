use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::level_state::LevelState;
use crate::levels::components::LevelDescription;

#[derive(Resource)]
pub struct LevelsResource {
    pub levels: HashMap<LevelState, LevelDescription>,
}
