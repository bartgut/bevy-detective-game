use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    LevelLoading,
    LevelSpriteLoading,
    InGame,
    GameOver,
}
