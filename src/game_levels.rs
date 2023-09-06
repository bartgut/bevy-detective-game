use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::level_state::LevelState;
use crate::levels::components::{LevelBundle, LevelDescription};

fn train_platform_level() -> (LevelState, LevelBundle) {
    (
        LevelState::TrainPlatform,
        LevelBundle {
            level_description: {
                LevelDescription {
                    level_name: String::from("train_platform"),
                    player_initial_position: Transform::from_translation(Vec3::new(
                        -600.0, -125.0, 0.0,
                    )),
                }
            },
            transform: Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)),
        },
    )
}

fn city_park_level() -> (LevelState, LevelBundle) {
    (
        LevelState::CityPark,
        LevelBundle {
            level_description: LevelDescription {
                level_name: String::from("city_park"),
                player_initial_position: Transform::from_translation(Vec3::new(
                    -600.0, -125.0, 0.0,
                )),
            },
            transform: Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)),
        },
    )
}

fn library_internals() -> (LevelState, LevelBundle) {
    (
        LevelState::LibraryInternals,
        LevelBundle {
            level_description: LevelDescription {
                level_name: String::from("library_internals"),
                player_initial_position: Transform::from_translation(Vec3::new(600.0, -125.0, 0.0)),
            },
            transform: Transform::from_translation(Vec3::new(300.0, 0.0, 0.0))
                .with_scale(Vec3::new(0.5, 0.5, 0.5)),
        },
    )
}

fn morgue() -> (LevelState, LevelBundle) {
    (
        LevelState::Morgue,
        LevelBundle {
            level_description: LevelDescription {
                level_name: String::from("morgue"),
                player_initial_position: Transform::from_translation(Vec3::new(600.0, -125.0, 0.0)),
            },
            transform: Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)),
        },
    )
}

/// ALL GAMES LEVEL
pub fn level_map() -> HashMap<LevelState, LevelBundle> {
    let mut map = HashMap::<LevelState, LevelBundle>::new();
    let (train_platform_state, train_platform_level_description) = train_platform_level();
    let (city_park_state, city_park_level_description) = city_park_level();
    let (library_state, library_level_description) = library_internals();
    let (morgue_state, morgue_description) = morgue();
    map.insert(train_platform_state, train_platform_level_description);
    map.insert(city_park_state, city_park_level_description);
    map.insert(library_state, library_level_description);
    map.insert(morgue_state, morgue_description);
    map
}
