use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game_state::GameState;
use crate::in_game_state::InGameState;
use crate::level_state::LevelState;
use crate::levels::constants::CAMERA_SPEED;
use crate::levels::resource::LevelsResource;
use super::components::*;

pub fn load_current_level(
    mut commands: Commands,
    current_level_state: Res<State<LevelState>>,
    mut game_state: ResMut<NextState<GameState>>,
    levels: Res<LevelsResource>,
) {
    match levels.levels.get(current_level_state.get()) {
        Some(level) => {
            commands.spawn(level.clone()).insert(CurrentLevel);
            game_state.set(GameState::LevelSpriteLoading);
        }
        None => {}
    }
}

pub fn initialize_current_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level_query: Query<(&LevelDescription, &Transform), With<CurrentLevel>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    match current_level_query.get_single() {
        Ok((level, transform)) => {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(format!("images/levels/{}.png", level.level_name)),
                    transform: transform.clone(),
                    ..default()
                },
                CurrentLevelSprite,
            ));
            game_state.set(GameState::InGame);
        }
        Err(_) => {}
    }
}

pub fn despawn_current_level(
    mut commands: Commands,
    query_level: Query<Entity, With<CurrentLevel>>,
    query_level_sprite: Query<Entity, With<CurrentLevelSprite>>,
) {
    for entity_level in query_level.iter() {
        for entity_level_sprite in query_level_sprite.iter() {
            commands.entity(entity_level_sprite).despawn_recursive();
            commands.entity(entity_level).remove::<CurrentLevel>();
        }
    }
}

pub fn keyboard_level_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut level_query: Query<&mut Transform, With<CurrentLevelSprite>>,
    time: Res<Time>,
) {
    for mut level_transform in level_query.iter_mut() {
        let mut translation = level_transform.translation;
        let time = time.delta_seconds();
        if keyboard_input.pressed(KeyCode::Left) {
            translation.x += CAMERA_SPEED * time;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            translation.x -= CAMERA_SPEED * time;
        }
        level_transform.translation = translation;
    }
}

pub fn keyboard_camera_move_blocked_when_border_reached(
    window_query: Query<&Window, With<PrimaryWindow>>,
    images: Res<Assets<Image>>,
    mut level_query: Query<(&Handle<Image>, &mut Transform), With<CurrentLevelSprite>>,
) {
    let window = window_query.get_single().unwrap();
    for (image, mut tranform) in level_query.iter_mut() {
        if let Some(image) = images.get(image) {
            let width_halved = image.texture_descriptor.size.width as f32 / 2.0;
            let res_x_halved = window.resolution.width() / 2.0;
            if width_halved - res_x_halved <= tranform.translation.x {
                tranform.translation.x = width_halved - res_x_halved;
            } else if res_x_halved - width_halved >= tranform.translation.x {
                tranform.translation.x = res_x_halved - width_halved;
            }
        }
    }
}

pub fn on_level_state_change(
    level_state: ResMut<NextState<LevelState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    if level_state.is_changed() {
        game_state.set(GameState::LevelLoading);
        in_game_state.set(InGameState::InGame);
    }
}

pub fn level_change_trigger_handler(
    mut commands: Commands,
    mut level_change_trigger_query: Query<(Entity, &LevelChangeTrigger)>,
    current_level_state: Res<State<LevelState>>,
    mut level_state: ResMut<NextState<LevelState>>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    if let Ok((entity, destination)) = level_change_trigger_query.get_single_mut() {
        commands.entity(entity).despawn_recursive();
        if &destination.level_state == current_level_state.get() {
            in_game_state.set(InGameState::InGame);
            return; // Cannot travel to the same location
        } else {
            level_state.set(destination.level_state);
        }
    }
}
