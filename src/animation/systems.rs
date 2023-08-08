use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::prelude::TimerMode::Repeating;
use crate::animation::components::{AnimationTimer, WalkingSettings};
use crate::movement::linear_movement::components::LinearMovementComponent;
use crate::player::components::Player;

pub fn walking_animation_trigger(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<
        (Entity, &mut Transform, &Sprite, &WalkingSettings),
        Added<LinearMovementComponent>,
    >,
) {
    for (entity, mut transform, mut sprite, settings) in query.iter_mut() {
        let texture_handle = asset_server.load(&settings.walking_sprite_texture);
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 114.0), 26, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands.entity(entity).remove::<Sprite>().insert((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_translation(transform.translation.xy().extend(1.0))
                    .with_scale(Vec3::new(1.5, 1.5, 1.0)),
                ..Default::default()
            },
            AnimationTimer(Timer::from_seconds(0.03, Repeating)),
        ));
        println!("Animation triggered")
    }
}

pub fn walking_animation_finished(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut removed: RemovedComponents<LinearMovementComponent>,
    mut query: Query<(Entity, &Transform), With<Player>>,
) {
    for _ in &mut removed {
        for (entity, &transform) in &query {
            commands
                .entity(entity)
                .remove::<SpriteSheetBundle>()
                .insert(SpriteBundle {
                    texture: asset_server.load("images/player/main.png"),
                    transform: transform.clone().with_scale(Vec3::new(1.0, 1.0, 1.0)),
                    ..default()
                });
        }
        println!("Animation finished")
    }
}

pub fn animation_executor(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        &WalkingSettings,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (settings, mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            sprite.index = if sprite.index == settings.last {
                settings.first
            } else {
                sprite.index + 1
            };
        }
    }
}

/*pub fn walking_animation_executor(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut )>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut movement) in query.iter_mut() {
        let delta = time.delta_seconds();
        let direction_vector_norm = (movement.to - transform.translation)
            .xy()
            .extend(0.0)
            .normalize();
        transform.translation += direction_vector_norm * movement.velocity * delta;
        if transform.translation.distance(movement.to) < 0.5 {
            commands.entity(entity).remove::<LinearMovementComponent>();
        }
    }
}*/
