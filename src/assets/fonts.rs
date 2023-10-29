use bevy::prelude::*;

#[derive(Resource)]
pub struct Fonts {
    pub noir_font_regular: Handle<Font>,
    pub noir_font_bold: Handle<Font>,
}

pub fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let noir_font_regular = asset_server.load("fonts/Noir_regular.ttf");
    let noir_font_bold = asset_server.load("fonts/Noir_bold.ttf");
    commands.insert_resource(Fonts {
        noir_font_regular: noir_font_regular,
        noir_font_bold: noir_font_bold,
    });
}
