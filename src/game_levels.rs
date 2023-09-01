use bevy::log::Level;
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

fn city_park_level() -> (LevelState, LevelDescription) {
    (
        LevelState::CityPark,
        LevelDescription {
            level_name: String::from("city_park"),
            level_initial_position: Vec3::new(300.0, 0.0, 0.0),
            player_initial_position: Vec3::new(-600.0, -125.0, 0.0),
        },
    )
}

fn library_internals() -> (LevelState, LevelDescription) {
    (
        LevelState::LibraryInternals,
        LevelDescription {
            level_name: String::from("library_internals"),
            level_initial_position: Vec3::new(300.0, 0.0, 0.0),
            player_initial_position: Vec3::new(-600.0, -125.0, 0.0),
        },
    )
}

/// ALL GAMES LEVEL
pub fn level_map() -> HashMap<LevelState, LevelDescription> {
    let mut map = HashMap::<LevelState, LevelDescription>::new();
    let (train_platfrom_state, train_platform_level_description) = train_platform_level();
    let (city_park_state, city_park_level_description) = city_park_level();
    let (library_state, library_level_description) = library_internals();
    map.insert(train_platfrom_state, train_platform_level_description);
    map.insert(city_park_state, city_park_level_description);
    map.insert(library_state, library_level_description);
    map
}
