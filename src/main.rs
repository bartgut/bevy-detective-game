pub mod dialogs;
pub mod game_levels;
pub mod game_npc;
pub mod game_state;
pub mod in_game_state;
pub mod level_state;
pub mod levels;
pub mod main_menu;
pub mod movement;
pub mod npc;
pub mod parsing;
pub mod player;

use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::dialogs::DialogsPlugin;
use crate::game_levels::level_map;
use crate::game_npc::npc_map;
use crate::game_state::GameState;
use crate::in_game_state::InGameState;
use crate::level_state::LevelState;
use crate::levels::LevelPlugin;
use crate::levels::resource::LevelsResource;
use crate::main_menu::MainMenuPlugin;
use crate::movement::MovementPlugin;
use crate::npc::NpcPlugin;
use crate::npc::resource::NPCResource;
use crate::player::PlayerPlugin;
use crate::State::{APPEARING, DISAPPEARING, NOT_VISIBLE, VISIBLE};

extern crate pest;
#[macro_use]
extern crate pest_derive;

////////////////////////////////////////////////////

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(LevelsResource {
            levels: level_map(),
        })
        .insert_resource(NPCResource { npcs: npc_map() })
        .add_state::<GameState>()
        .add_state::<InGameState>()
        .add_state::<LevelState>()
        .add_plugin(DialogsPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(MovementPlugin)
        .add_plugin(NpcPlugin)
        .add_startup_system(camera_setup)
        /*.add_startup_system(appearing_text_setup)
        .add_startup_system(type_writing_text_setup)
        .add_system(not_visible_setup)
        .add_system(appearing_setup)
        .add_system(visible_setup)
        .add_system(disappearing_setup)
        .add_system(type_writing_len_update)
        .add_system(type_writing_text_update)*/
        //        .add_system(test_dialog_func)
        //.add_startup_system(intro_setup)
        //.add_system(intro_text_show)
        .run()
}

//// To be moved later to separate files and add as a render type to the dialog runner

#[derive(Component)]
struct StartText;

#[derive(Component)]
struct AppearingTextSettings {
    not_visible_seconds: f32,
    appearing_seconds: f32,
    visible_seconds: f32,
    disappearing_seconds: f32,
}

impl Default for AppearingTextSettings {
    fn default() -> Self {
        AppearingTextSettings {
            not_visible_seconds: 1.0,
            appearing_seconds: 2.0,
            visible_seconds: 5.0,
            disappearing_seconds: 2.0,
        }
    }
}

#[derive(Component)]
struct TypeWritingTextSettings {
    text: String,
    every: f32,
    cur_len: usize,
}

#[derive(Component)]
struct TypeWritingTextTimer(Timer);

#[derive(Bundle)]
struct TypeWritingTextBundle {
    settings: TypeWritingTextSettings,
    timer: TypeWritingTextTimer,
    text: TextBundle,
}

fn type_writing_text_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let my_string = String::from("Hello, world!\nSecond line, third world");
    commands.spawn(create_type_writing_text(&my_string, 0.15, asset_server));
}

fn create_type_writing_text(
    text_to_show: &String,
    period: f32,
    asset_server: Res<AssetServer>,
) -> TypeWritingTextBundle {
    return TypeWritingTextBundle {
        timer: TypeWritingTextTimer(Timer::from_seconds(period, TimerMode::Repeating)),
        text: TextBundle::from_section(
            text_to_show,
            TextStyle {
                font: asset_server.load("fonts/Noir_regular.ttf"),
                font_size: 50.0,
                color: Color::WHITE.with_a(1.0),
            },
        )
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(400.0),
                right: Val::Px(600.0),
                ..default()
            },
            ..default()
        }),
        settings: TypeWritingTextSettings {
            text: text_to_show.clone(),
            every: period,
            cur_len: 0,
        },
    };
}

fn type_writing_len_update(
    mut type_writing_query: Query<(&mut TypeWritingTextSettings, &mut TypeWritingTextTimer)>,
    time: Res<Time>,
) {
    for (mut setting, mut timer) in type_writing_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            if setting.cur_len != setting.text.len() {
                setting.cur_len += 1;
            }
        }
    }
}

fn type_writing_text_update(mut query: Query<(&mut TypeWritingTextSettings, &mut Text)>) {
    for (mut settings, mut text) in query.iter_mut() {
        text.sections[0].value = settings.text.chars().take(settings.cur_len).collect()
    }
}

/* Appearing text */
#[derive(Component)]
struct NotVisibleTimer(Timer);

#[derive(Component)]
struct AppearingTimer(Timer);

#[derive(Component)]
struct VisibleTimer(Timer);

#[derive(Component)]
struct DisappearingTimer(Timer);

#[derive(Bundle)]
struct AppearingTextBundle {
    not_visible_timer: NotVisibleTimer,
    appearing_timer: AppearingTimer,
    visible_timer: VisibleTimer,
    disappearing_timer: DisappearingTimer,
    start_state: State,
    text: TextBundle,
}

#[derive(Component)]
enum State {
    NOT_VISIBLE,
    APPEARING,
    VISIBLE,
    DISAPPEARING,
    INVISIBLE,
}
////////////////////////////

fn appearing_text_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let appearingText = AppearingTextBundle {
        not_visible_timer: NotVisibleTimer(Timer::from_seconds(2.0, TimerMode::Once)),
        appearing_timer: AppearingTimer(Timer::from_seconds(3.0, TimerMode::Once)),
        visible_timer: VisibleTimer(Timer::from_seconds(5.0, TimerMode::Once)),
        disappearing_timer: DisappearingTimer(Timer::from_seconds(3.0, TimerMode::Once)),
        start_state: NOT_VISIBLE,
        text: TextBundle::from_section(
            "My super text",
            TextStyle {
                font: asset_server.load("fonts/Noir_regular.ttf"),
                font_size: 50.0,
                color: Color::WHITE.with_a(0.0),
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
    };
    commands.spawn(appearingText);
}

fn not_visible_setup(time: Res<Time>, mut query: Query<(&mut State, &mut NotVisibleTimer)>) {
    for (mut state, mut timer) in query.iter_mut() {
        match *state {
            NOT_VISIBLE => {
                if timer.0.tick(time.delta()).just_finished() {
                    *state = APPEARING;
                }
            }
            _ => (),
        }
    }
}

fn appearing_setup(
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut State, &mut AppearingTimer)>,
) {
    for (mut text, mut state, mut timer) in query.iter_mut() {
        match *state {
            APPEARING => {
                if timer.0.tick(time.delta()).just_finished() {
                    *state = VISIBLE;
                }
                text.sections[0].style.color = Color::WHITE
                    .with_a(1.0 / timer.0.duration().as_secs_f32() * timer.0.elapsed_secs())
            }
            _ => (),
        }
    }
}

fn visible_setup(time: Res<Time>, mut query: Query<(&mut State, &mut VisibleTimer)>) {
    for (mut state, mut timer) in query.iter_mut() {
        match *state {
            VISIBLE => {
                if timer.0.tick(time.delta()).just_finished() {
                    *state = DISAPPEARING;
                }
            }
            _ => (),
        }
    }
}

fn disappearing_setup(
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut State, &mut DisappearingTimer)>,
) {
    for (mut text, mut state, mut timer) in query.iter_mut() {
        match *state {
            DISAPPEARING => {
                if timer.0.tick(time.delta()).just_finished() {
                    *state = NOT_VISIBLE;
                }
                text.sections[0].style.color = Color::WHITE
                    .with_a(1.0 - 1.0 / timer.0.duration().as_secs_f32() * timer.0.elapsed_secs())
            }
            _ => (),
        }
    }
}

struct AppearingTextPlugin;

impl Plugin for AppearingTextPlugin {
    fn build(&self, app: &mut App) {}
}

fn intro_text_show(time: Res<Time>, mut query: Query<&mut Text, With<StartText>>) {
    let mut intro_text = query.single_mut();
    let seconds = time.elapsed_seconds();
    intro_text.sections[0].style.color =
        Color::WHITE.with_a(intro_text.sections[0].style.color.a() + 0.005 * seconds)
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn intro_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        TextBundle::from_section(
            "My super text",
            TextStyle {
                font: asset_server.load("fonts/Noir_regular.ttf"),
                font_size: 50.0,
                color: Color::WHITE.with_a(0.0),
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        StartText,
    ));
}
