use bevy::prelude::*;
use crate::comics::systems::ComicsSequence;
use crate::comics::vertical2images::components::{
    Vertical2Images, Vertical2ImagesDown, Vertical2ImagesFrame, Vertical2ImagesTop,
};
use crate::ui::components::InvisibleToVisibleTransition;

impl ComicsSequence for Vertical2Images {
    fn start_sequence(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        audio: &Res<Audio>,
    ) {
        if let Some(sound) = &self.comic_page_sound {
            audio.play(asset_server.load(sound));
        }
        if let Some(sound) = &self.top_image_sound {
            audio.play(asset_server.load(sound));
        }
        self.frame = Some(
            commands
                .spawn(create_vertical2images_frame())
                .insert(Vertical2ImagesFrame)
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(5.0)),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::BLACK.into(),
                        ..default()
                    });
                    self.loaded_top_image = Some(
                        parent
                            .spawn(add_image_to_frame(
                                self.top_image_path.clone(),
                                &asset_server,
                            ))
                            .insert(Vertical2ImagesTop)
                            .insert(InvisibleToVisibleTransition(Timer::from_seconds(
                                10.0,
                                TimerMode::Once,
                            )))
                            .id(),
                    );
                    self.loaded_bottom_image = Some(
                        parent
                            .spawn(add_image_to_frame(
                                self.bottom_image_path.clone(),
                                &asset_server,
                            ))
                            .insert(Vertical2ImagesDown)
                            .id(),
                    );
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(5.0)),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::BLACK.into(),
                        ..default()
                    });
                })
                .id(),
        )
    }

    fn next_frame(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        audio: &Res<Audio>,
    ) {
        if let Some(sound) = &self.down_image_sound {
            audio.play(asset_server.load(sound));
        }

        if let Some(bottom_image) = self.loaded_bottom_image {
            commands
                .entity(bottom_image)
                .insert(InvisibleToVisibleTransition(Timer::from_seconds(
                    10.0,
                    TimerMode::Once,
                )));
        }

        self.sequence_finished = true;
    }

    fn finished(&self) -> bool {
        self.sequence_finished
    }

    fn end_sequence(&mut self, commands: &mut Commands) {
        if let Some(frame) = self.frame {
            commands.entity(frame).despawn_recursive();
        }
    }
}

fn create_vertical2images_frame() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            gap: Size::new(Val::Percent(5.0), Val::Percent(5.0)),
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    }
}

fn add_image_to_frame(image_path: String, asset_server: &Res<AssetServer>) -> ImageBundle {
    ImageBundle {
        style: Style {
            size: Size::new(Val::Percent(50.0), Val::Percent(45.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        image: asset_server.load(image_path).into(),
        background_color: Color::rgba(1.0, 1.0, 1.0, 0.0).into(),
        ..default()
    }
}
