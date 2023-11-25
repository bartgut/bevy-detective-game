use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use rive_bevy::{GenericEvent, SceneTarget, SpriteEntity, StateMachine};
use crate::game_state::GameState;
use crate::level_state::LevelState;
use crate::ui::components::ButtonInteractionAction;
use super::components::*;

pub fn initialize_main_menu_ui(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    let mut main_menu_ui = Image::default();
    main_menu_ui.resize(Extent3d {
        width: 1280,
        height: 960,
        ..default()
    });
    let animation_image_handle = images.add(main_menu_ui.clone());

    let sprite_entity = commands
        .spawn(SpriteBundle {
            texture: animation_image_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        .insert(MainMenuUI)
        .id();
    let machine = StateMachine {
        riv: asset_server.load("rive/main_menu.riv"),
        handle: rive_bevy::Handle::Name("MainMenuStateMachine".into()),
        artboard_handle: rive_bevy::Handle::Name("MainMenu".into()),
        ..default()
    };

    commands
        .spawn(machine)
        .insert(SceneTarget {
            image: animation_image_handle,
            sprite: SpriteEntity {
                entity: Some(sprite_entity),
            },
            ..default()
        })
        .insert(MainMenuUI);
}

pub fn despawn_main_menu_ui(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn main_menu_interaction(
    mut rive_event: EventReader<GenericEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut level_state: ResMut<NextState<LevelState>>,
    mut exit: EventWriter<AppExit>,
) {
    for event in rive_event.read() {
        if event.name == "StartGame" {
            level_state.set(LevelState::TrainPlatform);
            game_state.set(GameState::Intro);
        } else if event.name == "QuitGame" {
            exit.send(AppExit);
        }
    }
}
