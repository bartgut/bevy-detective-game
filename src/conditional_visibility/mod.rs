use std::marker::PhantomData;
use bevy::prelude::*;
use bevy_yarnspinner::dialog_runner::context::StateContext;
use crate::conditional_visibility::systems::{visibility_on_added, visibility_on_state_change};
use crate::global_state::global_state::GlobalState;
use crate::inventory::components::Inventory;

pub mod components;
mod systems;

pub struct VisibilityConditionalPlugin;

impl Plugin for VisibilityConditionalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                visibility_on_added::<GlobalState>,
                visibility_on_state_change::<GlobalState>,
            ),
        );
    }
}
