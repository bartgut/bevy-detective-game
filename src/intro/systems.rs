use std::str::FromStr;
use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use crate::comics::rive::components::{RiveComics, RiveComicsPage, RiveComicsPageAudio, RiveComicsSink};
use crate::game_state::GameState;
use crate::intro_state::IntroState;
use crate::text::typewriting::components::{TextWithPause, TypeWritingTextSettings};
use crate::ui::components::FullScreenText;
use crate::ui::systems::full_screen_text;

pub fn start_intro(commands: Commands, asset_server: Res<AssetServer>) {
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
    mut commands: Commands,
    mouse_buttons: Res<Input<MouseButton>>,
    mut game_state: ResMut<NextState<GameState>>,
    intro_state: ResMut<State<IntroState>>,
    mut intro_state_mutator: ResMut<NextState<IntroState>>,
    mut comics_sink_query: Query<&mut RiveComicsSink>,
    mut page_audio: Query<Entity, With<RiveComicsPageAudio>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        match intro_state.get() {
            IntroState::TypewritingReport => {
                intro_state_mutator.set(IntroState::Comics1);
            }
            IntroState::Comics1 => {
                if let Some(mut comics_sink) = comics_sink_query.get_single_mut().ok() {
                    for entity in page_audio.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                    comics_sink.next_page(&mut commands);
                    if comics_sink.is_finished() {
                        intro_state_mutator.set(IntroState::End);
                    }
                }
            }
            IntroState::End => {
                game_state.set(GameState::LevelLoading);
            }
        }
    }
}

pub fn comics_start(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(RiveComics {
        rive_file_handle: asset_server.load("rive/comics.riv"),
        pages: vec![
            RiveComicsPage {
                artboard_name: "Comics 1 page".to_string(),
                animation_state_machine: "Page1SM".to_string(),
                events_handler: |commands, asset_server, event| {
                    if event.name == "Page1Start" {
                        commands
                            .spawn(dialog_bundle(
                                "sound/comics/Comics1Page1.ogg",
                                &asset_server,
                            ))
                            .insert(RiveComicsPageAudio);
                    }
                },
            },
            RiveComicsPage {
                artboard_name: "Comics 2 page".to_string(),
                animation_state_machine: "Page2SM".to_string(),
                events_handler: |commands, asset_server, event| {
                    if event.name == "Page2Start" {
                        commands
                            .spawn(dialog_bundle(
                                "sound/comics/Comics1Page2.ogg",
                                &asset_server,
                            ))
                            .insert(RiveComicsPageAudio);
                    }
                },
            },
            RiveComicsPage {
                artboard_name: "Comics 3 page".to_string(),
                animation_state_machine: "Page3SM".to_string(),
                events_handler: |commands, asset_server, event| {
                    if event.name == "Page3Start" {
                        commands
                            .spawn(dialog_bundle(
                                "sound/comics/Comics1Page3.ogg",
                                &asset_server,
                            ))
                            .insert(RiveComicsPageAudio);
                    }
                },
            },
            RiveComicsPage {
                artboard_name: "Comics 4 page".to_string(),
                animation_state_machine: "Page4SM".to_string(),
                events_handler: |commands, asset_server, event| {
                    if event.name == "Page4Start" {
                        commands
                            .spawn(dialog_bundle(
                                "sound/comics/Comics1Page4.ogg",
                                &asset_server,
                            ))
                            .insert(RiveComicsPageAudio);
                    }
                },
            },
        ],
    });
}

fn dialog_bundle(path: &str, asset_server: &Res<AssetServer>) -> AudioBundle {
    AudioBundle {
        source: asset_server.load(String::from_str(path).unwrap()),
        settings: PlaybackSettings {
            mode: PlaybackMode::Remove,
            ..default()
        },
        ..default()
    }
}
