use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::clickable::items::behaviour::ClickableBehaviour;
use crate::clickable::items::ClickableItem;
use crate::level_state::LevelState;

#[derive(Resource)]
pub struct ClickableItemResource<T: ClickableBehaviour + Component> {
    pub items: HashMap<LevelState, Vec<ClickableItem<T>>>,
}
