use bevy::prelude::*;
use crate::comics::systems::ComicsSequence;

#[derive(Bundle, Copy, Clone)]
pub struct SinglePageComicsBundle<T: ComicsSequence + Component + Clone> {
    pub sequence: T,
}

#[derive(Bundle)]
pub struct MultiPageComicsBundle<T: ComicsSequence + Component + Clone> {
    pub pages: ComicsPages<T>,
}

#[derive(Component)]
pub struct ComicsPages<T: ComicsSequence + Component + Clone> {
    pub pages: Vec<SinglePageComicsBundle<T>>,
    pub current_page: usize,
}

impl<T: ComicsSequence + Component + Clone> ComicsPages<T> {
    pub fn get_current_page(&self) -> SinglePageComicsBundle<T> {
        self.pages[self.current_page].clone()
    }

    pub fn next_page(&mut self) -> Option<SinglePageComicsBundle<T>> {
        self.current_page += 1;
        if self.current_page >= self.pages.len() {
            None
        } else {
            Some(self.pages[self.current_page].clone())
        }
    }
}
