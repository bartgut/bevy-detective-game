use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum ComicsState {
    #[default]
    IDLE,
    ONGOING,
    END,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MultiPageComicsState {
    #[default]
    IDLE,
    ONGOING,
    END,
}
