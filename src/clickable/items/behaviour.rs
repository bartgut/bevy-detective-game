use bevy::prelude::*;

pub trait ClickableBehaviour {
    fn on_start(&mut self, commands: &mut Commands, asset_server: Res<AssetServer>);
    fn on_click(&mut self, commands: &mut Commands, asset_server: Res<AssetServer>);
    fn on_close(&mut self, commands: &mut Commands);
}
