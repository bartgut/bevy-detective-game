use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum DialogInternalState {
    #[default]
    NoDialog,
    DialogAvatarLoading,
    Dialog,
}
