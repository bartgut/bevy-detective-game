use bevy::prelude::*;
pub trait AudioPlayable {
    fn play(&self, commands: &mut Commands, asset_server: &Res<AssetServer>);
}
