use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res};
use bevy::prelude::*;
use crate::comics::components::ComicsPages;
use crate::comics::vertical2images::components::Vertical2ImagesFrame;
use crate::comics_state::{ComicsState, MultiPageComicsState};

pub trait ComicsSequence {
    fn start_sequence(
        &mut self,
        command: &mut Commands,
        asset_server: &Res<AssetServer>,
        audio: &Res<Audio>,
    );
    fn next_frame(
        &mut self,
        command: &mut Commands,
        asset_server: &Res<AssetServer>,
        audio: &Res<Audio>,
    );
    fn finished(&self) -> bool;
    fn end_sequence(&mut self);
}

pub fn comics_inserted<T: ComicsSequence + Component>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut comics_state: ResMut<NextState<ComicsState>>,
    mut query: Query<&mut T, Added<T>>,
) {
    for mut entity in query.iter_mut() {
        comics_state.set(ComicsState::ONGOING);
        entity.start_sequence(&mut commands, &asset_server, &audio);
    }
}

pub fn multi_page_comics_inserted<T: ComicsSequence + Component + Clone>(
    mut commands: Commands,
    mut comics_state: ResMut<NextState<MultiPageComicsState>>,
    mut query: Query<&ComicsPages<T>, Added<ComicsPages<T>>>,
) {
    for mut comics_pages in query.iter_mut() {
        comics_state.set(MultiPageComicsState::ONGOING);
        commands.spawn(comics_pages.get_current_page());
    }
}

pub fn multi_page_comics_next_page<T: ComicsSequence + Component + Clone>(
    mut commands: Commands,
    mut comics_state: ResMut<NextState<MultiPageComicsState>>,
    single_page_comics_state: Res<State<ComicsState>>,
    mut query: Query<(Entity, &mut ComicsPages<T>)>,
) {
    match single_page_comics_state.0 {
        ComicsState::END => {
            for (entity, mut comics_pages) in query.iter_mut() {
                match comics_pages.next_page() {
                    None => {
                        comics_state.set(MultiPageComicsState::END);
                        commands.entity(entity).despawn_recursive();
                    }
                    Some(next_page) => {
                        commands.spawn(next_page);
                        ()
                    }
                }
            }
        }
        _ => {}
    }
}

pub fn comics_next_frame<T: ComicsSequence + Component>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut T)>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut comics_state: ResMut<NextState<ComicsState>>,
    audio: Res<Audio>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        for (entity, mut comics_seq) in query.iter_mut() {
            if comics_seq.finished() == false {
                comics_seq.next_frame(&mut commands, &asset_server, &audio);
            } else {
                comics_seq.end_sequence();
                comics_state.set(ComicsState::END);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn comics_cleanup(mut commands: Commands, query: Query<Entity, With<Vertical2ImagesFrame>>) {
    // TODO - deleting a bundle?
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
