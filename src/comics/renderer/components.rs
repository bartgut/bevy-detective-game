use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res};

pub trait ComicsRenderer {
    fn current_render(&self, commands: &mut Commands, asset_server: &Res<AssetServer>);
    fn move_to_next_frame(&mut self);
    fn finished(&self) -> bool;
    fn clear(&self, commands: &mut Commands);
}
