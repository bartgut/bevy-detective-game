use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::animation::components::AnimationEnabled;
use crate::clickable::components::Clickable;
use crate::npc::components::{DialogableNPC, NPC, SpawnableNPC};

#[derive(Component)]
pub struct Librarian;

impl SpawnableNPC for Librarian {
    fn spawn(&self, level: &mut EntityCommands, asset_server: &Res<AssetServer>) {
        level.with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: asset_server.load("images/npc/librarian/librarian.png"),
                    transform: Transform::from_translation(Vec3::new(-1000.0, -60.0, 1.0)),
                    ..default()
                },
                NPC {
                    texture_file: String::from("librarian.png"),
                    level_initial_position: Vec3::new(-1000.0, -600.0, 1.0),
                },
                Clickable {
                    required_distance: 150.0,
                },
                DialogableNPC {
                    dialog_handle: asset_server.load("dialogs/first_dialog.yarn"),
                    start_node: String::from("Librarian1PlayerConversationIntro"),
                    reset_node: String::from("Librarian1PlayerPossibleQuestions"),
                    first_dialog_mark: String::from("librarian_already_talked"),
                },
                AnimationEnabled,
                Librarian,
            ));
        });
    }
}
