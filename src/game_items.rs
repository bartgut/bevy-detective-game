use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::clickable::components::Clickable;
use crate::clickable::items::behaviour::ClickableBehaviour;
use crate::clickable::items::ClickableItem;
use crate::clickable::items::onesideitem::OneSideItem;
use crate::clickable::items::twosideitem::{TextureSide, TwoSideItem};
use crate::level_state::LevelState;
use crate::npc::components::{DialogableNPC, NPC};

fn letter_from_a_young_lover() -> ClickableItem<TwoSideItem> {
    ClickableItem::<TwoSideItem> {
        item: TwoSideItem::new_with_dialog(
            "letter/young_lover_letter_front.png".to_string(),
            "letter/young_lover_letter_back.png".to_string(),
            "Player".to_string(),
            "Wydaje sie troche za mloda jak na zone nieboszczyka".to_string(),
        ),
        clickable: Clickable {
            clickable_texture: "letter/young_lover_mini.png".to_string(),
            level_initial_position: Vec3::new(-800.0, -120.0, 1.0),
        },
    }
}

pub fn two_side_items_map() -> HashMap<LevelState, Vec<ClickableItem<TwoSideItem>>> {
    let mut map = HashMap::<LevelState, Vec<ClickableItem<TwoSideItem>>>::new();
    let letter = letter_from_a_young_lover();
    map.entry(LevelState::TrainPlatform)
        .or_insert(vec![])
        .push(letter);
    map
}

pub fn one_side_items_map() -> HashMap<LevelState, Vec<ClickableItem<OneSideItem>>> {
    let mut map = HashMap::<LevelState, Vec<ClickableItem<OneSideItem>>>::new();
    map
}
