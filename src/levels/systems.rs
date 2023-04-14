use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::level_state::LevelState;
use super::components::*;

pub fn initialize_train_platform_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut level_state: ResMut<NextState<LevelState>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/levels/train_platform.png"),
            transform: Transform::from_xyz(600.0, 0.0, 0.0), // move to struct with a start_position field
            ..default()
        },
        TrainPlatformLevel,
    ));
    level_state.set(LevelState::TrainPlatform);
}

pub fn despawn_train_platform_level(
    mut commands: Commands,
    query: Query<Entity, With<TrainPlatformLevel>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// TODO controlling pace based on frames
pub fn keyboard_level_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut level_query: Query<&mut Transform, With<TrainPlatformLevel>>,
    time: Res<Time>,
) {
    for mut level_transform in level_query.iter_mut() {
        let mut translation = level_transform.translation;
        if keyboard_input.pressed(KeyCode::Left) {
            translation.x += 2.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            translation.x -= 2.0;
        }
        level_transform.translation = translation;
    }
}

pub fn keyboard_camera_move_blocked_when_border_reached(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut level_query: Query<&mut Transform, With<TrainPlatformLevel>>,
) {
    let window = window_query.get_single().unwrap();
    for (mut tranform) in level_query.iter_mut() {
        let width_halved = 2688.0 / 2.0; // TODO
        let res_x_halved = window.resolution.width() / 2.0;
        if width_halved - res_x_halved <= tranform.translation.x {
            tranform.translation.x = width_halved - res_x_halved;
        }
        if res_x_halved - width_halved >= tranform.translation.x {
            tranform.translation.x = res_x_halved - width_halved;
        }
    }
}
