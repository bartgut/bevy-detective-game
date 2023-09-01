use bevy::prelude::*;
use crate::clickable::components::Clickable;
use crate::clickable::items::behaviour::ClickableBehaviour;

pub mod behaviour;
pub mod components;
pub mod onesideitem;
pub mod resource;
pub mod twosideitem;
#[derive(Bundle)]
pub struct ClickableItem<T: ClickableBehaviour + Component> {
    pub item: T,
    pub clickable: Clickable,
}
