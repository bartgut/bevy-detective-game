use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::animation::components::{AnimationEnabled};
use crate::levels::components::{CurrentLevel, CurrentLevelSprite, LevelDescription};
use crate::player::components::{Player};
use crate::movement::linear_movement::components::Linear2DMovementComponent;
use crate::player::animation::idle_animation::IdleAnimation;
use crate::player::animation::walking_animation::WalkingAnimation;
use crate::player::constants::PLAYER_SPEED;

pub fn initialize_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_sprite_query: Query<Entity, With<CurrentLevelSprite>>,
    level_query: Query<&LevelDescription, With<CurrentLevel>>,
) {
    let level_entity = level_sprite_query.get_single().unwrap();
    let level_description = level_query.get_single().unwrap();
    commands.entity(level_entity).with_children(|parent| {
        parent.spawn((
            SpriteBundle {
                texture: asset_server.load("images/player/main.png"),
                transform: level_description.player_initial_position.clone(),
                ..default()
            },
            Player,
            WalkingAnimation,
            IdleAnimation,
            AnimationEnabled,
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
    level_query: Query<&Transform, (Without<Player>, With<CurrentLevelSprite>)>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
) {
    let width_halved = 2688.0 / 2.0; // TODO
    let window = window_query.get_single().unwrap();
    for level_transform in level_query.iter() {
        for (entity, transform) in player_query.iter() {
            if let Some(position) = window.cursor_position() {
                if mouse_input.just_pressed(MouseButton::Left) {
                    let new_level_position = (width_halved - level_transform.translation.x)
                        + (position.x - window.resolution.width() / 2.0);
                    commands.entity(entity).insert(Linear2DMovementComponent {
                        to: Vec2::new(-width_halved + new_level_position, transform.translation.y),
                        velocity: Vec2::ONE * PLAYER_SPEED,
                    });
                }
            }
        }
    }
}
