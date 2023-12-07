use bevy::prelude::*;
use crate::clickable::components::Clickable;
use crate::clickable::items::behaviour::ClickableBehaviour;

pub mod behaviour;
pub mod components;
pub mod event_handler;
pub mod onesideitem;
pub mod resource;
pub mod rive_item;
pub mod systems;
pub mod twosideitem;

#[derive(Bundle)]
pub struct ClickableItem<T: ClickableBehaviour + Component> {
    pub item: T,
    pub clickable: Clickable,
}
