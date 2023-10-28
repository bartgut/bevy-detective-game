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
    quests: Query<&Quest>,
    mut game_state_mutator: ResMut<NextState<InGameState>>,
) {
    if keyboard_buttons.just_pressed(KeyCode::M) {
        world_map_handle(&current_game_state, &mut game_state_mutator);
    }
    if keyboard_buttons.just_pressed(KeyCode::I) {
        inventory_handle(&inventory);
    }
    if keyboard_buttons.just_pressed(KeyCode::Q) {
        quest_handle(&quests);
    }
}

fn quest_handle(quests: &Query<&Quest>) {
    let (active_quests, completed_quests): (Vec<_>, Vec<_>) = quests.iter()
        .filter(|quest| quest.status == QuestStatus::Active || quest.status == QuestStatus::Complete)
        .partition(|quest| quest.status == QuestStatus::Active);

    println!("Active quests:");
    active_quests.iter().for_each(|quest| {
        println!("Quest: {:?} - {:?}", quest.short_description, quest.long_description);
    });
    println!("Completed quests:");
    completed_quests.iter().for_each(|quest| {
        println!("Quest: {:?} - {:?}", quest.short_description, quest.long_description);
    });
}

fn world_map_handle(
    current_game_state: &Res<State<InGameState>>,
    game_state_mutator: &mut ResMut<NextState<InGameState>>,
) {
    match current_game_state.get() {
        InGameState::InGame => {
            game_state_mutator.set(InGameState::Map);
        }
        InGameState::Map => {
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
