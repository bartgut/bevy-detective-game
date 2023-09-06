use bevy::prelude::*;
pub trait AudioPlayable {
    fn play(&self, asset_server: &Res<AssetServer>, audio: &Res<Audio>);
}
