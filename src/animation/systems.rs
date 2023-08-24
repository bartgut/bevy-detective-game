use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::prelude::TimerMode::Repeating;
use crate::animation::components::{AnimationEnabled, AnimationTimer, SpriteAnimationSettings};

pub fn animation_on_added_component<
    Who: Component,
    AnimationSettings: SpriteAnimationSettings + Component,
    Trigger: Component,
>(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<
        (Entity, &mut Transform, &AnimationSettings),
        (With<Who>, With<AnimationEnabled>, Added<Trigger>),
    >,
) {
    for (entity, transform, animation_settings) in query.iter_mut() {
        create_animation(
            &mut commands,
            &mut asset_server,
            &mut texture_atlases,
            entity,
            &transform,
            animation_settings,
        )
    }
}

pub fn animation_on_removed_component<
    Who: Component,
    AnimationSettings: SpriteAnimationSettings + Component,
    Trigger: Component,
>(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut removed: RemovedComponents<Trigger>,
    query: Query<(Entity, &Transform, &AnimationSettings), With<Who>>,
) {
    for removed_entity in &mut removed {
        for (entity, &transform, animation_settings) in &query {
            if entity == removed_entity {
                create_animation(
                    &mut commands,
                    &mut asset_server,
                    &mut texture_atlases,
                    entity,
                    &transform,
                    animation_settings,
                )
            }
        }
    }
}

pub fn animation_executor<
    Who: Component,
    AnimationSettings: SpriteAnimationSettings + Component,
>(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationSettings,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<Who>,
    >,
) {
    for (settings, mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            sprite.index = if sprite.index == settings.get_last() {
                settings.get_first()
            } else {
                sprite.index + 1
            };
        }
    }
}

fn create_animation(
    commands: &mut Commands,
    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    entity: Entity,
    transform: &Transform,
    animation_settings: &dyn SpriteAnimationSettings,
) {
    let texture_handle = asset_server.load(&animation_settings.get_grid_info().file_path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        animation_settings.get_grid_info().tile_size,
        animation_settings.get_grid_info().columns,
        animation_settings.get_grid_info().rows,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.entity(entity).remove::<Sprite>().insert((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(transform.translation.xy().extend(1.0))
                .with_scale(animation_settings.get_grid_info().tile_scale),
            ..Default::default()
        },
        AnimationTimer(Timer::from_seconds(
            animation_settings.animation_pace(),
            Repeating,
        )),
    ));
}
