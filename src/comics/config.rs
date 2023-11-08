use bevy::prelude::*;
#[derive(Component, Clone)]
pub struct ComicsPage {
    pub template: ComicsTemplate,
    pub whole_page_sounds: Option<String>,
}

#[derive(Clone)]
pub struct ComicsFrame {
    pub image_path: String,
    pub sound_path: Option<String>,
}

#[derive(Clone)]
pub enum ComicsTemplate {
    Vertical2Images {
        top_frame: ComicsFrame,
        bottom_frame: ComicsFrame,
    },
}

#[derive(Component)]
pub struct ComicsPages {
    pub pages: Vec<ComicsPage>,
    pub current_page: usize,
}

impl ComicsPages {
    pub fn spawn_current_page(&self, commands: &mut Commands) {
        commands.spawn(self.pages[self.current_page].clone());
    }

    pub fn next_page(&mut self) -> Option<&ComicsPage> {
        self.current_page += 1;
        self.pages.get(self.current_page)
    }
}
