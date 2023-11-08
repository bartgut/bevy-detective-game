use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::comics::renderer::components::ComicsRenderer;

#[derive(Component)]
pub struct ComicsSink {
    pub assets: HashMap<String, Handle<Image>>,
    pub renderer: Box<dyn ComicsRenderer + Send + Sync>,
}
