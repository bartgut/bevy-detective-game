use bevy::app::App;
use bevy::prelude::*;
use bevy::prelude::{AssetServer, Component, NextState, Query, Res, ResMut, States};

pub trait AssetLoadingMonitor {
    fn loaded(&self, asset_server: &Res<AssetServer>) -> bool;
}

pub trait AssetLoadingStateChangeExt {
    fn add_component_loading_state<T: Component + AssetLoadingMonitor, S: States>(
        &mut self,
        from: S,
        to: S,
    ) -> &mut Self;

    fn add_resource_loading_state<T: Resource + AssetLoadingMonitor, S: States>(
        &mut self,
        from: S,
        to: S,
    ) -> &mut Self;
}

impl AssetLoadingStateChangeExt for App {
    fn add_component_loading_state<T: Component + AssetLoadingMonitor, S: States>(
        &mut self,
        from: S,
        to: S,
    ) -> &mut Self {
        let func = move |query: Query<&T>,
                         asset_server: Res<AssetServer>,
                         mut state: ResMut<NextState<S>>| {
            if let Some(entity) = query.get_single().ok() {
                if entity.loaded(&asset_server) {
                    state.set(to.clone());
                }
            }
        };
        self.add_systems(Update, func.run_if(in_state(from)))
    }

    fn add_resource_loading_state<T: Resource + AssetLoadingMonitor, S: States>(
        &mut self,
        from: S,
        to: S,
    ) -> &mut Self {
        let func = move |asset: Res<T>,
                         asset_server: Res<AssetServer>,
                         mut state: ResMut<NextState<S>>| {
            if asset.loaded(&asset_server) {
                state.set(to.clone());
            }
        };
        self.add_systems(Update, func.run_if(in_state(from)))
    }
}
