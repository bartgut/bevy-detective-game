pub mod animation;
pub mod clickable;
pub mod comics;
pub mod comics_state;
pub mod dialogs;
pub mod game_items;
pub mod game_levels;
pub mod game_npc;
pub mod game_state;
pub mod in_game_state;
pub mod intro;
pub mod intro_state;
pub mod level_state;
pub mod levels;
pub mod main_menu;
pub mod movement;
pub mod npc;
pub mod parsing;
pub mod player;
pub mod sound;
pub mod text;
pub mod ui;

use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::animation::SpriteAnimationPlugin;
use crate::clickable::ClickablePlugin;
use crate::clickable::items::onesideitem::OneSideItem;
use crate::clickable::items::resource::ClickableItemResource;
use crate::clickable::items::twosideitem::TwoSideItem;
use crate::comics::ComicsPlugin;
use crate::comics_state::{ComicsState, MultiPageComicsState};
use crate::dialogs::DialogsPlugin;
use crate::game_items::{one_side_items_map, two_side_items_map};
use crate::game_levels::level_map;
use crate::game_npc::npc_map;
use crate::game_state::GameState;
use crate::in_game_state::InGameState;
use crate::intro::IntroPlugin;
use crate::intro_state::IntroState;
use crate::level_state::LevelState;
use crate::levels::LevelPlugin;
use crate::levels::resource::LevelsResource;
use crate::main_menu::MainMenuPlugin;
use crate::movement::MovementPlugin;
use crate::npc::NpcPlugin;
use crate::npc::resource::NPCResource;
use crate::player::PlayerPlugin;
use crate::State::{APPEARING, DISAPPEARING, NOT_VISIBLE, VISIBLE};
use crate::text::TypeWritingTextPlugin;
use crate::ui::UIUtilsPlugin;

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
        .insert_resource(ClickableItemResource::<OneSideItem> {
            items: one_side_items_map(),
        })
        .insert_resource(ClickableItemResource::<TwoSideItem> {
            items: two_side_items_map(),
        })
        .add_state::<IntroState>()
        .add_state::<GameState>()
        .add_state::<InGameState>()
        .add_state::<LevelState>()
        .add_state::<ComicsState>()
        .add_state::<MultiPageComicsState>()
        .add_plugin(DialogsPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(IntroPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(MovementPlugin)
        .add_plugin(NpcPlugin)
        .add_plugin(TypeWritingTextPlugin)
        .add_plugin(ClickablePlugin)
        .add_plugin(ComicsPlugin)
        .add_plugin(UIUtilsPlugin)
        .add_plugin(SpriteAnimationPlugin)
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
