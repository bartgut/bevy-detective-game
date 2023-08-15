use bevy::prelude::*;
use crate::comics::systems::{ComicsSequence};

#[derive(Bundle)]
pub struct MultiPageComicsBundle {
    pub pages: ComicsPages,
}

#[derive(Component)]
pub struct ComicsPages {
    pub pages: Vec<Box<dyn ComicsSequence + Send + Sync>>,
    pub current_page: usize,
}

impl ComicsPages {
    pub fn get_current_page(&mut self) -> &mut Box<dyn ComicsSequence + Send + Sync> {
        &mut self.pages[self.current_page]
    }

    pub fn next_page(&mut self) -> Option<&mut Box<dyn ComicsSequence + Send + Sync>> {
        self.current_page += 1;
        self.pages.get_mut(self.current_page)
    }
}
