use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use rive_bevy::{GenericEvent, SceneTarget, SpriteEntity, StateMachine};
use crate::comics::rive::components::{RiveComics, RiveComicsSink, RiveComicsUI};

pub fn initialize_comics(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    comics_query: Query<&RiveComics, Added<RiveComics>>,
) {
    if let Some(comics) = comics_query.get_single().ok() {
        let comics_first_page = &comics.pages[0];
        let mut comics_ui = Image::default();
        let rive_file = asset_server.load(comics.rive_file.clone());
        comics_ui.resize(Extent3d {
            width: 1280,
            height: 960,
            ..default()
        });
        let animation_image_handle = images.add(comics_ui.clone());

        let comics_sprite = commands
            .spawn(SpriteBundle {
                texture: animation_image_handle.clone(),
                transform: Transform::from_scale(Vec3::splat(1.0)),
                ..Default::default()
            })
            .insert(RiveComicsUI)
            .id();

        let machine = StateMachine {
            riv: rive_file.clone(),
            handle: rive_bevy::Handle::Name(
                comics_first_page.animation_state_machine.clone().into(),
            ),
            artboard_handle: rive_bevy::Handle::Name(
                comics_first_page.artboard_name.clone().into(),
            ),
            ..default()
        };

        commands
            .spawn(machine)
            .insert(SceneTarget {
                image: animation_image_handle.clone(),
                sprite: SpriteEntity {
                    entity: Some(comics_sprite),
                },
                ..default()
            })
            .insert(RiveComicsUI)
            .insert(RiveComicsSink::new(
                rive_file.clone(),
                animation_image_handle.clone(),
                comics_sprite,
                comics.pages.clone(),
            ));
    }
}

pub fn comics_interaction_events(
    mut commands: Commands,
    mut rive_event: EventReader<GenericEvent>,
    asset_server: Res<AssetServer>,
    mut comics_sink_query: Query<&mut RiveComicsSink>,
) {
    if let Some(comics_sink) = comics_sink_query.get_single_mut().ok() {
        for event in rive_event.read() {
            comics_sink.event_handle(&mut commands, &asset_server, event);
        }
    }
}

pub fn despawn_comics(mut commands: Commands, query: Query<Entity, With<RiveComicsUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
