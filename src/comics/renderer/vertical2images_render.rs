use bevy::audio::PlaybackMode::Once;
use bevy::prelude::*;
use crate::comics::components::ComicsSink;
use crate::comics::renderer::components::ComicsRenderer;
use crate::comics::renderer::vertical2images_render::Vertical2ImageRenderCurrent::{
    BOTTOM, END, START, TOP,
};
use crate::ui::components::InvisibleToVisibleTransition;

pub enum Vertical2ImageRenderCurrent {
    START,
    TOP,
    BOTTOM,
    END,
}

#[derive(Component)]
pub struct Vertical2ImagesRenderer {
    frame: Entity,
    entity_top: Entity,
    entity_bottom: Entity,
    frame_counter: Vertical2ImageRenderCurrent,
    audio_frame: Option<String>,
    audio_top_image: Option<String>,
    audio_bottom_image: Option<String>,
}

impl Vertical2ImagesRenderer {
    pub fn init(
        commands: &mut Commands,
        top_image: Handle<Image>,
        bottom_image: Handle<Image>,
        audio_frame: Option<String>,
        audio_top_image: Option<String>,
        audio_bottom_image: Option<String>,
    ) -> Self {
        let mut entity_top = Entity::PLACEHOLDER;
        let mut entity_bottom = Entity::PLACEHOLDER;
        let frame = commands
            .spawn(create_vertical2images_frame())
            .with_children(|parent| {
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(5.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                });
                entity_top = parent.spawn(add_image_to_frame(top_image)).id();
                entity_bottom = parent.spawn(add_image_to_frame(bottom_image)).id();
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(5.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                });
            })
            .id();

        Vertical2ImagesRenderer {
            frame,
            entity_top,
            entity_bottom,
            frame_counter: START,
            audio_frame,
            audio_top_image,
            audio_bottom_image,
        }
    }
}
impl ComicsRenderer for Vertical2ImagesRenderer {
    fn current_render(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        match &self.frame_counter {
            TOP => {
                commands
                    .entity(self.entity_top)
                    .insert(InvisibleToVisibleTransition(Timer::from_seconds(
                        10.0,
                        TimerMode::Once,
                    )));
                play_audio(self.frame, &self.audio_frame, commands, asset_server);
                play_audio(
                    self.entity_top,
                    &self.audio_top_image,
                    commands,
                    asset_server,
                );
            }
            BOTTOM => {
                commands
                    .entity(self.entity_bottom)
                    .insert(InvisibleToVisibleTransition(Timer::from_seconds(
                        10.0,
                        TimerMode::Once,
                    )));
                play_audio(
                    self.entity_bottom,
                    &self.audio_bottom_image,
                    commands,
                    asset_server,
                )
            }
            _ => {}
        }
    }

    fn move_to_next_frame(&mut self) {
        match &self.frame_counter {
            START => self.frame_counter = TOP,
            TOP => self.frame_counter = BOTTOM,
            BOTTOM => self.frame_counter = END,
            _ => {}
        }
    }

    fn finished(&self) -> bool {
        match &self.frame_counter {
            END => true,
            _ => false,
        }
    }

    fn clear(&self, commands: &mut Commands) {
        commands.entity(self.frame).despawn_recursive();
    }
}

impl ComicsSink {
    pub fn current_render(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        self.renderer.current_render(commands, &asset_server);
    }
    pub fn move_to_next_frame(&mut self) {
        self.renderer.move_to_next_frame();
    }
    pub fn finished(&self) -> bool {
        self.renderer.finished()
    }

    pub fn clear(&self, commands: &mut Commands) {
        self.renderer.clear(commands);
    }
}

fn create_vertical2images_frame() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Percent(5.0),
            column_gap: Val::Percent(5.0),
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    }
}

fn add_image_to_frame(image_handle: Handle<Image>) -> ImageBundle {
    ImageBundle {
        style: Style {
            width: Val::Percent(50.0),
            height: Val::Percent(45.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        image: image_handle.into(),
        background_color: Color::rgba(1.0, 1.0, 1.0, 0.0).into(),
        ..default()
    }
}

fn play_audio(
    entity: Entity,
    audio: &Option<String>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    if let Some(audio) = audio {
        commands.entity(entity).insert(AudioBundle {
            source: asset_server.load(audio),
            settings: PlaybackSettings {
                mode: Once,
                ..default()
            },
        });
    }
}
