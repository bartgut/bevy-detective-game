use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::animation::components::AnimationEnabled;
use crate::clickable::components::Clickable;
use crate::game::npc::railwayman::animation::smoking_animation::SmokingAnimation;
use crate::npc::components::{DialogableNPC, NPC, SpawnableNPC};

#[derive(Component)]
pub struct Railwayman;

impl SpawnableNPC for Railwayman {
    fn spawn(&self, level: &mut EntityCommands, asset_server: &Res<AssetServer>) {
        level.with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: asset_server.load("images/npc/railwayman/railwayman.png"),
                    transform: Transform::from_translation(Vec3::new(800.0, -60.0, 1.0))
                        .with_scale(Vec3::new(1.3, 1.3, 1.0)),
                    ..default()
                },
                NPC {
                    texture_file: String::from("railwayman/railwayman.png"),
                    level_initial_position: Vec3::new(800.0, -60.0, 1.0),
                },
                DialogableNPC {
                    dialog_handle: asset_server.load("dialogs/railwayman_dialog.yarn"),
                    start_node: String::from("RailwaymanDialogIntro"),
                    reset_node: String::from("RailwaymanDialogPossibleQuestions"),
                    first_dialog_mark: String::from("railwayman_already_talked"),
                },
                Clickable {
                    required_distance: 150.0,
                },
                AnimationEnabled,
                SmokingAnimation,
                Railwayman,
            ));
        });
    }
}
