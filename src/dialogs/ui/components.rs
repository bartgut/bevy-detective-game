use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use bevy_yarnspinner::asset::asset::YarnSpinnerDialog;
use bevy_yarnspinner::parsing::components::LineType;
use crate::assets::asset_loading_monitor::AssetLoadingMonitor;

#[derive(Component)]
pub struct DialogUI;

#[derive(Component)]
pub struct DialogUIText;

#[derive(Component)]
pub struct DialogUIImage;

#[derive(Component)]
pub struct OptionUI;

#[derive(Component)]
pub struct OptionUINode {
    pub node_title: String,
    pub used: bool,
}
#[derive(Resource, Default)]
pub struct AvatarHandles {
    pub handles: HashMap<String, Handle<Image>>,
}

impl AvatarHandles {
    pub fn add_from_dialog(&mut self, dialog: &YarnSpinnerDialog, asset_server: &Res<AssetServer>) {
        for node in &dialog.nodes {
            for line in node.lines.iter() {
                match line {
                    LineType::DialogLine { speaker, .. } => {
                        self.get_weak_or_add(speaker, asset_server);
                    }
                    LineType::OptionLine { speaker, .. } => {
                        self.get_weak_or_add(speaker, asset_server);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn get_weak_or_add(&mut self, key: &str, asset_server: &Res<AssetServer>) -> Handle<Image> {
        self.handles
            .get(key)
            .map(|handle| handle.clone_weak())
            .unwrap_or(self.load(key, asset_server))
    }

    pub fn load(&mut self, key: &str, asset_server: &Res<AssetServer>) -> Handle<Image> {
        let handle = asset_server.load(format!("images/avatars/{}.png", key));
        self.handles.insert(key.to_string(), handle.clone());
        handle.clone_weak()
    }
}

impl AssetLoadingMonitor for AvatarHandles {
    fn loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        self.handles
            .iter()
            .all(|(_, handle)| asset_server.get_load_state(handle) == Some(LoadState::Loaded))
    }
}
