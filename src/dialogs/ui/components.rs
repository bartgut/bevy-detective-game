use bevy::prelude::*;
use bevy::utils::HashMap;

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
    pub fn get_weak_or_add(&mut self, key: &str, asset_server: &Res<AssetServer>) -> Handle<Image> {
        if let Some(handle) = self.handles.get(key) {
            return handle.clone_weak();
        }
        self.load(key, asset_server)
    }

    pub fn load(&mut self, key: &str, asset_server: &Res<AssetServer>) -> Handle<Image> {
        let handle = asset_server.load(format!("images/avatars/{}.png", key));
        self.handles.insert(key.to_string(), handle.clone());
        handle.clone_weak()
    }
}
