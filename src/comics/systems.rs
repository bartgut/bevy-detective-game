use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res};
use bevy::prelude::*;
use crate::comics::components::ComicsPages;
use crate::comics_state::MultiPageComicsState;

pub trait ComicsSequence {
    fn start_sequence(&mut self, command: &mut Commands, asset_server: &Res<AssetServer>);
    fn next_frame(&mut self, command: &mut Commands, asset_server: &Res<AssetServer>);
    fn finished(&self) -> bool;
    fn end_sequence(&mut self, command: &mut Commands);
}

pub fn multi_page_comics_inserted(
    mut commands: Commands,
    mut comics_state: ResMut<NextState<MultiPageComicsState>>,
    mut query: Query<&mut ComicsPages, Added<ComicsPages>>,
    asset_server: Res<AssetServer>,
) {
    for mut comics_pages in query.iter_mut() {
        comics_state.set(MultiPageComicsState::ONGOING);
        comics_pages
            .get_current_page()
            .start_sequence(&mut commands, &asset_server);
    }
}

pub fn comics_next_frame(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut ComicsPages)>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut comics_state: ResMut<NextState<MultiPageComicsState>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        for (entity, mut comics_pages) in query.iter_mut() {
            let current_page = comics_pages.get_current_page();
            if !current_page.finished() {
                comics_pages
                    .get_current_page()
                    .next_frame(&mut commands, &asset_server);
            } else {
                current_page.end_sequence(&mut commands);
                match comics_pages.next_page() {
                    None => {
                        comics_state.set(MultiPageComicsState::END);
                        commands.entity(entity).despawn_recursive();
                    }
                    Some(mut next_page) => next_page.start_sequence(&mut commands, &asset_server),
                }
            }
        }
    }
}
