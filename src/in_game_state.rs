use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum InGameState {
    #[default]
    InGame,
    LookingAtItem,
    QuestLog,
    Map,
    Dialog,
}
