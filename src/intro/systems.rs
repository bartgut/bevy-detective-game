use bevy::app::AppLabel;
use bevy::prelude::*;
use crate::game_state::GameState;
use crate::text::typewriting::components::{TextWithPause, TypeWritingTextSettings};
use crate::ui::components::FullScreenText;
use crate::ui::systems::full_screen_text;

pub fn start_intro(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("sound/background/intro.ogg"));
    full_screen_text(
        &mut commands,
        asset_server,
        vec![
            TextWithPause {
                textSettings: TypeWritingTextSettings {
                    text: "Komenda miejsca w miescie P.\n\
                           Mlodszy inspektor Michal Pawlak\n\n\n\n"
                        .to_string(),
                    every: 0.1,
                    randomizing: 0.02,
                    cur_len: 0,
                },
                pause: 0.0,
            },
            TextWithPause {
                textSettings: TypeWritingTextSettings {
                    text: "15 grudnia 2023\n\n".to_string(),
                    every: 0.1,
                    randomizing: 0.02,
                    cur_len: 0,
                },
                pause: 1.5,
            },
            TextWithPause {
                textSettings: TypeWritingTextSettings {
                    text: "W miescie P. nieznany sprawcy dokonal zabojstwa 41 letniego mezczyzny Lucjana Baranowskiego,\n\
                           zwloki denata zostaly znalezione na platformie kolejowej przed miejska biblioteka około godziny 9 wieczorem.\n\
                           Poki co nie zostalo ustalone czy jest to miejsce zbrodni czy tez zwloki zostaly tutaj przetransportowane.\n\
                           Przyczyna smierci rowniez nie zostala ustalona. ".to_string(),
                    every: 0.1,
                    randomizing: 0.02,
                    cur_len: 0,
                },
                pause: 1.5,
            },
        ],
    );
}

pub fn intro_cleanup(
    mut commands: Commands,
    intro_text_query: Query<Entity, With<FullScreenText>>,
) {
    for entity in intro_text_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn mouse_interaction(
    mouse_buttons: Res<Input<MouseButton>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        game_state.set(GameState::LevelLoading);
    }
}
