use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res};
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::comics::components::ComicsSink;
use crate::comics::config::{ComicsPage, ComicsPages, ComicsTemplate};
use crate::comics::renderer::vertical2images_render::Vertical2ImagesRenderer;
use crate::comics_state::MultiPageComicsState;

pub fn comics_page_inserted(
    mut commands: Commands,
    mut query: Query<(Entity, &ComicsPage), Added<ComicsPage>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, comics_page_settings) in query.iter_mut() {
        let sink = config_to_sink(comics_page_settings, &mut commands, &asset_server);
        commands.entity(entity).insert(sink);
    }
}

pub fn comics_page_mouse_interaction(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut ComicsSink>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        for mut comics_sink in query.iter_mut() {
            comics_sink.move_to_next_frame();
            comics_sink.current_render(&mut commands, &asset_server);
        }
    }
}

pub fn multi_page_comics_inserted(
    mut commands: Commands,
    mut comics_state: ResMut<NextState<MultiPageComicsState>>,
    mut query: Query<&mut ComicsPages, Added<ComicsPages>>,
) {
    for mut comics_pages in query.iter_mut() {
        comics_state.set(MultiPageComicsState::ONGOING);
        comics_pages.spawn_current_page(&mut commands);
    }
}

pub fn multi_page_comics_next_page(
    mut commands: Commands,
    mut comics_state: ResMut<NextState<MultiPageComicsState>>,
    mut query: Query<&mut ComicsPages>,
    mut current_comics_page: Query<(Entity, &ComicsSink)>,
) {
    for mut comics_pages in query.iter_mut() {
        for (entity, comics_page) in current_comics_page.iter_mut() {
            if comics_page.finished() {
                comics_page.clear(&mut commands);
                commands.entity(entity).despawn_recursive();
                match comics_pages.next_page() {
                    Some(_page) => {
                        comics_pages.spawn_current_page(&mut commands);
                    }
                    None => {
                        comics_state.set(MultiPageComicsState::END);
                    }
                }
            }
        }
    }
}

fn config_to_sink(
    page: &ComicsPage,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> ComicsSink {
    match &page.template {
        ComicsTemplate::Vertical2Images {
            top_frame: top,
            bottom_frame: bottom,
        } => {
            let mut assets = HashMap::<String, Handle<Image>>::new(); // TODO preload + state check
            ComicsSink {
                assets,
                renderer: Box::new(Vertical2ImagesRenderer::init(
                    commands,
                    asset_server.load(top.image_path.clone()),
                    asset_server.load(bottom.image_path.clone()),
                    page.whole_page_sounds.clone(),
                    top.sound_path.clone(),
                    bottom.sound_path.clone(),
                )),
            }
        }
    }
}
