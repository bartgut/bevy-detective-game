use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::player::components::Player;
use crate::levels::components::TrainPlatformLevel;
use crate::movement::linear_movement::components::LinearMovementComponent;

pub fn initialize_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_query: Query<Entity, With<TrainPlatformLevel>>,
) {
    let level = level_query.get_single().unwrap();
    commands.entity(level).with_children(|parent| {
        parent.spawn((
            SpriteBundle {
                texture: asset_server.load("images/player/main.png"),
                transform: Transform::from_xyz(-600.0, -140.0, 0.0), // move to struct with a start_position field
                ..default()
            },
            Player,
        ));
    });
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn cursor_position(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<Input<MouseButton>>,
    level_query: Query<&Transform, (Without<Player>, With<TrainPlatformLevel>)>,
    mut player_query: Query<Entity, With<Player>>,
) {
    let width_halved = 2688.0 / 2.0; // TODO
    let window = window_query.get_single().unwrap();
    for level_transform in level_query.iter() {
        for entity in player_query.iter() {
            if let Some(position) = window.cursor_position() {
                if mouse_input.just_pressed(MouseButton::Left) {
                    let new_level_position = (width_halved - level_transform.translation.x)
                        + (position.x - window.resolution.width() / 2.0);
                    commands.entity(entity).insert(LinearMovementComponent {
                        to: Vec3::new(-width_halved + new_level_position, -140.0, 0.0),
                        velocity: Vec3::ONE * 128.0,
                    });
                }
            }
        }
    }
}
