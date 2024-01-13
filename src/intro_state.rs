use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum IntroState {
    #[default]
    TypewritingReport,
    Comics1Loading,
    Comics1,
    End,
}
