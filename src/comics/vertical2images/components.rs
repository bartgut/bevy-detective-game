use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Vertical2Images {
    pub top_image_path: String,
    pub bottom_image_path: String,
    pub loaded_top_image: Option<Entity>,
    pub loaded_bottom_image: Option<Entity>,
    pub frame: Option<Entity>,
    pub sequence_finished: bool,
    pub comic_page_sound: Option<String>,
    pub top_image_sound: Option<String>,
    pub down_image_sound: Option<String>,
}

impl Vertical2Images {
    pub fn new(
        // images
        top_image_path: String,
        bottom_image_path: String,
        // sound
        comic_page_sound: Option<String>,
        top_image_sound: Option<String>,
        down_image_sound: Option<String>,
    ) -> Self {
        Self {
            top_image_path,
            bottom_image_path,
            loaded_top_image: None,
            loaded_bottom_image: None,
            frame: None,
            sequence_finished: false,
            comic_page_sound,
            top_image_sound,
            down_image_sound,
        }
    }
}

#[derive(Component)]
pub struct Vertical2ImagesTop;
#[derive(Component)]
pub struct Vertical2ImagesDown;
#[derive(Component)]
pub struct Vertical2ImagesFrame;
