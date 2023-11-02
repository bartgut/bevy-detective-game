use bevy::text::Text;
use bevy::prelude::*;
use bevy::audio::PlaybackMode::Despawn;

#[derive(Component)]
pub struct InvisibleToVisibleTransition(pub Timer);

#[derive(Component)]
pub struct FullScreenText;

#[derive(Component)]
pub struct ButtonInteractionAction<T> {
    pub on_none: fn(&mut Commands, asset_server: &Res<AssetServer>, &mut T),
    pub on_hover: fn(&mut Commands, asset_server: &Res<AssetServer>, &mut T),
    pub on_pressed: fn(&mut Commands, asset_server: &Res<AssetServer>, &mut T),
}

impl Default for ButtonInteractionAction<Text> {
    fn default() -> Self {
        Self {
            on_none: |commands, asset_server, text| text.sections[0].style.color = Color::BLACK,
            on_hover: |commands, asset_server, text| {
                text.sections[0].style.color = Color::RED;
                commands.spawn(AudioBundle {
                    source: asset_server.load(format!("sound/ui/button_hover.ogg")),
                    settings: PlaybackSettings {
                        mode: Despawn,
                        ..default()
                    },
                    ..default()
                });
            },
            on_pressed: |commands, asset_server, text| {
                commands.spawn(AudioBundle {
                    source: asset_server.load(format!("sound/ui/button_clicked.ogg")),
                    settings: PlaybackSettings {
                        mode: Despawn,
                        ..default()
                    },
                    ..default()
                });
            },
        }
    }
}
