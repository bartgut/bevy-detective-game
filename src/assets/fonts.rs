use bevy::prelude::*;

#[derive(Resource)]
pub struct Fonts {
    pub noir_font: Handle<Font>
}

pub fn load_fonts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let noir_font = asset_server.load("fonts/Noir_regular.ttf");
    commands.insert_resource(Fonts { noir_font });
}