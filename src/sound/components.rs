use bevy::prelude::*;
pub trait AudioPlayable {
    fn play(&self, commands: &mut Commands, asset_server: &Res<AssetServer>);
}

pub trait UIInteractionSoundEffect {
    fn on_hover(&self, commands: &mut Commands, asset_server: &Res<AssetServer>);
    fn on_pressed(&self, commands: &mut Commands, asset_server: &Res<AssetServer>);
}
