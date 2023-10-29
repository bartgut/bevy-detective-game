use bevy::prelude::*;
use crate::global_state::global_state::GlobalState;
use crate::in_game_state::InGameState;
use crate::inventory::components::Inventory;
use crate::quests::components::{Quest, QuestStatus};

pub fn keyboard_event(
    _: Commands,
    keyboard_buttons: Res<Input<KeyCode>>,
    current_game_state: Res<State<InGameState>>,
    inventory: Res<GlobalState>,
    mut game_state_mutator: ResMut<NextState<InGameState>>,
) {
    if keyboard_buttons.just_pressed(KeyCode::M) {
        in_game_state_mutator(
            &current_game_state,
            &mut game_state_mutator,
            InGameState::Map,
        );
    }
    if keyboard_buttons.just_pressed(KeyCode::Q) {
        in_game_state_mutator(
            &current_game_state,
            &mut game_state_mutator,
            InGameState::QuestLog,
        );
    }
    if keyboard_buttons.just_pressed(KeyCode::I) {
        inventory_handle(&inventory);
    }
}

fn in_game_state_mutator(
    current_game_state: &Res<State<InGameState>>,
    game_state_mutator: &mut ResMut<NextState<InGameState>>,
    target_state: InGameState,
) {
    match current_game_state.get() {
        InGameState::InGame => {
            game_state_mutator.set(target_state);
        }
        target_state => {
            game_state_mutator.set(InGameState::InGame);
        }
        _ => {}
    }
}

fn inventory_handle(inventory: &GlobalState) {
    println!("Inventory:");
    inventory.get_all_items().iter().for_each(|item| {
        println!("Item: {:?} - {:?}", item.name, item.description);
    });
}
