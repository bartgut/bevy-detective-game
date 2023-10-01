use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::level_state::LevelState;
use crate::levels::components::CurrentLevelSprite;
use crate::npc::resource::NPCResource;
use crate::player::components::Player;
use super::components::*;

pub fn initialize_npcs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_query: Query<Entity, With<CurrentLevelSprite>>,
    level_state: Res<State<LevelState>>,
    levels: Res<NPCResource>,
) {
    let level = level_query.get_single().unwrap();
    for npc in levels.npcs.get(level_state.get()).unwrap_or(&vec![]) {
        npc.spawn(&mut commands.entity(level), &asset_server)
    }
}

pub fn despawn_npc(mut commands: Commands, query: Query<Entity, With<NPC>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
