use bevy::prelude::*;
use crate::levels::components::TrainPlatformLevel;
use crate::player::components::Player;
use super::components::*;

pub fn initialize_npc(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_query: Query<Entity, With<TrainPlatformLevel>>) {
    let level = level_query.get_single().unwrap();
    commands.entity(level)
        .with_children(|parent| {
            parent.spawn(
                (SpriteBundle {
                    texture: asset_server.load("images/npc/librarian.png"),
                    transform: Transform::from_xyz(-100.0, -120.0, 0.0), // move to struct with a start_position field
                    ..default()
                }, NPC, DialogableNPC)
            );
        });
}

pub fn despawn_npc(
    mut commands: Commands,
    query: Query<Entity, With<NPC>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn dialogable_npc_can_start_dialog(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    npc_query: Query<(Entity, &Transform), With<DialogableNPC>>) {

    for player_transform in player_query.iter() {
        for (entity, npc_transform) in npc_query.iter() {
            if player_transform.translation.distance(npc_transform.translation) < 150.0 {
                commands.entity(entity).insert(CanStartDialog);
            } else {
                commands.entity(entity).remove::<CanStartDialog>();
            }
        }
    }

}