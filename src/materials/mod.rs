use bevy::prelude::Plugin;
use bevy::sprite::Material2dPlugin;
use bevy::prelude::*;
use crate::materials::fog::FogMaterial;

pub mod fog;

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<FogMaterial>::default());
    }
}
