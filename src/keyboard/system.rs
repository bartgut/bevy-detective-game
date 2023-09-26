use bevy::prelude::*;
use crate::global_state::global_state::GlobalState;
use crate::in_game_state::InGameState;
use crate::inventory::components::Inventory;

pub fn keyboard_event(
    _: Commands,
    keyboard_buttons: Res<Input<KeyCode>>,
    current_game_state: Res<State<InGameState>>,
    inventory: Res<GlobalState>,
    mut game_state_mutator: ResMut<NextState<InGameState>>,
) {
    if keyboard_buttons.just_pressed(KeyCode::M) {
        if current_game_state.get() == &InGameState::Map {
            game_state_mutator.set(InGameState::InGame);
        } else {
            game_state_mutator.set(InGameState::Map);
        }
    }
    if keyboard_buttons.just_pressed(KeyCode::I) {
        println!("Inventory:");
        inventory.get_all_items().iter().for_each(|item| {
            println!("Item: {:?} - {:?}", item.name, item.description);
        });
    }
}
