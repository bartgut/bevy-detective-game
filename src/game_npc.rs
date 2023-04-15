use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::level_state::LevelState;
use crate::npc::components::{DialogableNPC, NPC};

fn train_level_npc_1() -> (NPC, DialogableNPC) {
    (
        NPC {
            texture_file: String::from("librarian.png"),
            level_initial_position: Vec3::new(-1000.0, -120.0, 1.0),
        },
        DialogableNPC {
            dialog_file_name: String::from("first_dialog"),
            start_node: String::from("Librarian1PlayerConversationIntro"),
            reset_node: String::from("Librarian1PlayerPossibleQuestions"),
        },
    )
}

fn train_level_npc_2() -> (NPC, DialogableNPC) {
    (
        NPC {
            texture_file: String::from("railwayman.png"),
            level_initial_position: Vec3::new(-100.0, -150.0, 1.0),
        },
        DialogableNPC {
            dialog_file_name: String::from("first_dialog"),
            start_node: String::from("Librarian1PlayerConversationIntro"),
            reset_node: String::from("Librarian1PlayerPossibleQuestions"),
        },
    )
}

// ALL NPCS FOR ALL LEVELS

pub fn npc_map() -> HashMap<LevelState, Vec<(NPC, DialogableNPC)>> {
    let mut map = HashMap::<LevelState, Vec<(NPC, DialogableNPC)>>::new();
    let (npc, dialogable_npc) = train_level_npc_1();
    map.entry(LevelState::TrainPlatform)
        .or_insert(vec![])
        .push((npc, dialogable_npc));
    let (npc, dialogable_npc) = train_level_npc_2();
    map.entry(LevelState::TrainPlatform)
        .or_insert(vec![])
        .push((npc, dialogable_npc));
    map
}
