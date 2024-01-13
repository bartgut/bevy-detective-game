use bevy::prelude::*;
use crate::game_state::GameState;

pub mod asset_loading_monitor;
pub mod fonts;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, fonts::load_fonts)
            .add_systems(OnEnter(GameState::InLevelSpritesLoading), to_in_game_state);
    }
}

pub fn to_in_game_state(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::InGame);
}
