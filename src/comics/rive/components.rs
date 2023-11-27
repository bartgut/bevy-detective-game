use bevy::asset::AssetLoader;
use bevy::asset::saver::AssetSaver;
use bevy::prelude::*;
use rive_bevy::{GenericEvent, Riv, StateMachine};
use crate::assets::AssetsPlugin;

#[derive(Component)]
pub struct RiveComicsUI;

#[derive(Component)]
pub struct RiveComics {
    pub rive_file: String,
    pub pages: Vec<RiveComicsPage>,
}

#[derive(Component, Clone)]
pub struct RiveComicsPage {
    pub artboard_name: String,
    pub animation_state_machine: String,
    pub events_handler: fn(&mut Commands, &Res<AssetServer>, &GenericEvent),
}

#[derive(Component)]
pub struct RiveComicsSink {
    rive_file_handle: Handle<Riv>,
    animation_image_handle: Handle<Image>,
    sprite_entity: Entity,
    pages: Vec<RiveComicsPage>,
    current_page: usize,
}

impl RiveComicsSink {
    pub fn new(
        rive_file_handle: Handle<Riv>,
        animation_image_handle: Handle<Image>,
        sprite_entity: Entity,
        pages: Vec<RiveComicsPage>,
    ) -> Self {
        Self {
            rive_file_handle,
            animation_image_handle,
            sprite_entity,
            pages,
            current_page: 0,
        }
    }
    pub fn next_page(&mut self, commands: &mut Commands) {
        self.current_page += 1;

        if let Some(page) = self.pages.get(self.current_page) {
            let machine = StateMachine {
                riv: self.rive_file_handle.clone(),
                handle: rive_bevy::Handle::Name(page.animation_state_machine.clone().into()),
                artboard_handle: rive_bevy::Handle::Name(page.artboard_name.clone().into()),
                ..default()
            };
            commands.entity(self.sprite_entity).insert(machine);
        }
    }

    pub fn is_finished(&self) -> bool {
        if self.current_page >= self.pages.len() {
            true
        } else {
            false
        }
    }

    pub fn event_handle(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        event: &GenericEvent,
    ) {
        if let Some(page) = self.pages.get(self.current_page) {
            (page.events_handler)(commands, asset_server, event);
        }
    }
}
