use bevy::asset::AssetServer;
use bevy::prelude::{Commands, NodeBundle, Res};
use bevy::prelude::*;
use crate::comics::vertical2images::components::Vertical2ImagesFrame;
use crate::comics_state::ComicsState;
use crate::ui::components::InvisibleToVisibleTransition;

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

pub fn comics_next_frame<T: ComicsSequence + Component>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut T>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut comics_state: ResMut<NextState<ComicsState>>,
    audio: Res<Audio>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        for mut entity in query.iter_mut() {
            if entity.finished() == false {
                entity.next_frame(&mut commands, &asset_server, &audio);
            } else {
                entity.end_sequence();
                comics_state.set(ComicsState::END);
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
