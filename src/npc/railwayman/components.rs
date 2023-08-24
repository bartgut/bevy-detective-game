use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::animation::components::AnimationEnabled;
use crate::npc::components::{DialogableNPC, NPC, SpawnableNPC};
use crate::npc::railwayman::animation::smoking_animation::SmokingAnimation;

#[derive(Component)]
pub struct Railwayman;

impl SpawnableNPC for Railwayman {
    fn spawn(&self, level: &mut EntityCommands, asset_server: &Res<AssetServer>) {
        level.with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: asset_server.load("images/npc/railwayman/railwayman.png"),
                    transform: Transform::from_translation(Vec3::new(-100.0, -130.0, 1.0)),
                    ..default()
                },
                NPC {
                    texture_file: String::from("railwayman/railwayman.png"),
                    level_initial_position: Vec3::new(-100.0, -130.0, 1.0),
                },
                DialogableNPC {
                    dialog_file_name: String::from("railwayman_dialog"),
                    start_node: String::from("RailwaymanDialogIntro"),
                    reset_node: String::from("RailwaymanDialogPossibleQuestions"),
                    first_dialog: true,
                },
                AnimationEnabled,
                SmokingAnimation,
                Railwayman,
            ));
        });
    }
}
