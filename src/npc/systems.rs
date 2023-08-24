use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::animation::components::AnimationEnabled;
use crate::level_state::LevelState;
use crate::levels::components::CurrentLevelSprite;
use crate::npc::railwayman::animation::smoking_animation::SmokingAnimation;
use crate::npc::resource::NPCResource;
use crate::player::components::Player;
use super::components::*;

pub fn initialize_npcs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_query: Query<Entity, With<CurrentLevelSprite>>,
    levels: Res<NPCResource>,
) {
    let level = level_query.get_single().unwrap();
    for npc in levels.npcs.get(&LevelState::TrainPlatform).unwrap() {
        npc.spawn(&mut commands.entity(level), &asset_server)
    }
}

pub fn despawn_npc(mut commands: Commands, query: Query<Entity, With<NPC>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn dialogable_npc_can_start_dialog(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    npc_query: Query<(Entity, &Transform), With<DialogableNPC>>,
) {
    for player_transform in player_query.iter() {
        for (entity, npc_transform) in npc_query.iter() {
            if player_transform
                .translation
                .distance(npc_transform.translation)
                < 150.0
            {
                commands.entity(entity).insert(CanStartDialog);
            } else {
                commands.entity(entity).remove::<CanStartDialog>();
            }
        }
    }
}

pub fn print_when_hovered_over_npc(
    mut commands: Commands,
    mut cursor_evr: EventReader<CursorMoved>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    level_query: Query<&Transform, (Without<Player>, With<CurrentLevelSprite>)>,
    npc_query: Query<(Entity, &Transform), With<CanStartDialog>>,
) {
    let width_halved = 2688.0 / 2.0; // TODO
    let mut window = window_query.get_single_mut().unwrap();
    for ev in cursor_evr.iter() {
        for (entity, npc_transform) in npc_query.iter() {
            for level_transform in level_query.iter() {
                let new_level_position = (width_halved - level_transform.translation.x)
                    + (ev.position.x - window.resolution.width() / 2.0);
                let npc_position_2d = width_halved + npc_transform.translation.xy();
                if Vec2::new(new_level_position, 0.0).distance(Vec2::new(npc_position_2d.x, 0.0))
                    < 30.0
                {
                    window.cursor.icon = CursorIcon::Hand;
                    commands.entity(entity).insert(HoveredOverNPC);
                } else {
                    window.cursor.icon = CursorIcon::Default;
                    commands.entity(entity).remove::<HoveredOverNPC>();
                }
            }
        }
    }
}
