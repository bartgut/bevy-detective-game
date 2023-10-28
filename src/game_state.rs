use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Intro,
    LevelLoading,
    LevelSpriteLoading,
    InLevelSpritesLoading,
    InGame,
    GameOver,
}
