use bevy::prelude::*;
use crate::comics::components::{ComicsPages, MultiPageComicsBundle};
use crate::comics::vertical2images::components::Vertical2Images;
use crate::comics_state::MultiPageComicsState;
use crate::game_state::GameState;
use crate::intro_state::IntroState;
use crate::level_state::LevelState;
use crate::text::typewriting::components::{TextWithPause, TypeWritingTextSettings};
use crate::ui::components::FullScreenText;
use crate::ui::systems::full_screen_text;

pub fn start_intro(commands: Commands, asset_server: Res<AssetServer>) {
    //audio.play(asset_server.load("sound/background/intro.ogg"));
    full_screen_text(
        commands,
        asset_server,
        vec![
            TextWithPause {
                text_settings: TypeWritingTextSettings {
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
                text_settings: TypeWritingTextSettings {
                    text: "15 grudnia 2023\n\n".to_string(),
                    every: 0.1,
                    randomizing: 0.02,
                    cur_len: 0,
                },
                pause: 1.5,
            },
            TextWithPause {
                text_settings: TypeWritingTextSettings {
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

pub fn full_text_typewriting_cleanup(
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
    intro_state: ResMut<State<IntroState>>,
    mut intro_state_mutator: ResMut<NextState<IntroState>>,
    comics_state: Res<State<MultiPageComicsState>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        match intro_state.get() {
            IntroState::TypewritingReport => {
                intro_state_mutator.set(IntroState::Comics1);
            }
            IntroState::Comics1 => {
                if comics_state.get() == &MultiPageComicsState::END {
                    intro_state_mutator.set(IntroState::End);
                }
            }
            IntroState::End => {
                game_state.set(GameState::LevelLoading);
            }
        }
    }
}

pub fn comics_start(mut commands: Commands) {
    commands.spawn(MultiPageComicsBundle {
        pages: ComicsPages {
            pages: vec![
                Box::new(Vertical2Images::new(
                    "images/comics/intro/1/1.png".to_string(),
                    "images/comics/intro/1/2.png".to_string(),
                    Some("sound/background/rain_inside_the_car_1min.ogg".to_string()),
                    Some("sound/intro/car_approaching.ogg".to_string()),
                    Some("sound/intro/on_place.ogg".to_string()),
                )),
                Box::new(Vertical2Images::new(
                    "images/comics/intro/2/1.png".to_string(),
                    "images/comics/intro/2/2.png".to_string(),
                    Some("sound/background/rain_inside_the_car_1min.ogg".to_string()),
                    Some("sound/intro/car_approaching.ogg".to_string()),
                    Some("sound/intro/on_place.ogg".to_string()),
                )),
            ],
            current_page: 0,
        },
    });
}
