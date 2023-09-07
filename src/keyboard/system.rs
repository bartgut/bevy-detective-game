use bevy::prelude::*;
use crate::in_game_state::InGameState;

pub fn keyboard_event(
    commands: Commands,
    keyboard_buttons: Res<Input<KeyCode>>,
    current_game_state: Res<State<InGameState>>,
    mut game_state_mutator: ResMut<NextState<InGameState>>,
) {
    if keyboard_buttons.just_pressed(KeyCode::M) {
        if current_game_state.0 == InGameState::Map {
            game_state_mutator.set(InGameState::InGame);
        } else {
            game_state_mutator.set(InGameState::Map);
        }
    }
}
