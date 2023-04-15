use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::level_state::LevelState;
use crate::levels::components::LevelDescription;

fn train_platform_level() -> (LevelState, LevelDescription) {
    (
        LevelState::TrainPlatform,
        LevelDescription {
            level_name: String::from("train_platform"),
            level_initial_position: Vec3::new(300.0, 0.0, 0.0),
            player_initial_position: Vec3::new(-600.0, -125.0, 0.0),
        },
    )
}

/// ALL GAMES LEVEL
pub fn level_map() -> HashMap<LevelState, LevelDescription> {
    let mut map = HashMap::<LevelState, LevelDescription>::new();
    let (state, level_description) = train_platform_level();
    map.insert(state, level_description);
    map
}
