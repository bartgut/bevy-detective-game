use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::math::Vec3Swizzles;
use crate::clickable::components::{CanBeClicked, Clickable, Clicked, HoveredOverClickable};
use crate::clickable::items::behaviour::ClickableBehaviour;
use crate::clickable::items::resource::ClickableItemResource;
use crate::in_game_state::InGameState;
use crate::level_state::LevelState;
use crate::levels::components::CurrentLevelSprite;
use crate::player::components::Player;
use super::components;

pub fn initialize_clickable<T: ClickableBehaviour + Component + Clone>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_query: Query<Entity, With<CurrentLevelSprite>>,
    items_in_levels: Res<ClickableItemResource<T>>,
) {
    let level = level_query.get_single().unwrap();
    commands.entity(level).with_children(|parent| {
        for item_bundle in items_in_levels
            .items
            .get(&LevelState::TrainPlatform)
            .unwrap_or(&vec![])
        {
            parent.spawn((
                SpriteBundle {
                    texture: asset_server.load(format!(
                        "images/items/{}",
                        item_bundle.clickable.clickable_texture
                    )),
                    transform: Transform::from_translation(
                        item_bundle.clickable.level_initial_position,
                    ),
                    ..default()
                },
                Clickable {
                    clickable_texture: item_bundle.clickable.clickable_texture.clone(),
                    level_initial_position: item_bundle.clickable.level_initial_position,
                },
                item_bundle.item.clone(),
            ));
        }
    });
}

pub fn print_when_hovered_clickable(
    mut commands: Commands,
    mut cursor_evr: EventReader<CursorMoved>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    level_query: Query<&Transform, (Without<Player>, With<CurrentLevelSprite>)>,
    clickable_query: Query<(Entity, &Transform), With<CanBeClicked>>,
) {
    let width_halved = 2688.0 / 2.0; // TODO
    let mut window = window_query.get_single_mut().unwrap();
    for ev in cursor_evr.iter() {
        for (entity, clickable_transform) in clickable_query.iter() {
            for level_transform in level_query.iter() {
                let new_level_position = (width_halved - level_transform.translation.x)
                    + (ev.position.x - window.resolution.width() / 2.0);
                let npc_position_2d = width_halved + clickable_transform.translation.xy();
                if Vec2::new(new_level_position, 0.0).distance(Vec2::new(npc_position_2d.x, 0.0))
                    < 30.0
                {
                    window.cursor.icon = CursorIcon::Hand;
                    commands.entity(entity).insert(HoveredOverClickable);
                } else {
                    window.cursor.icon = CursorIcon::Default;
                    commands.entity(entity).remove::<HoveredOverClickable>();
                }
            }
        }
    }
}

pub fn clickable_can_be_clicked(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    clickable_query: Query<(Entity, &Transform), With<Clickable>>,
) {
    for player_transform in player_query.iter() {
        for (entity, clickable_transform) in clickable_query.iter() {
            if player_transform
                .translation
                .distance(clickable_transform.translation)
                < 150.0
            {
                commands.entity(entity).insert(CanBeClicked);
            } else {
                commands.entity(entity).remove::<CanBeClicked>();
            }
        }
    }
}

pub fn clickable_first_click<T: ClickableBehaviour + Component>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<InGameState>>,
    mut clickable: Query<(Entity, &mut T), With<HoveredOverClickable>>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if (!clickable.is_empty()) {
            let (entity, mut behaviour) = clickable.get_single_mut().unwrap();
            game_state.set(InGameState::LookingAtItem);
            commands.entity(entity).insert(Clicked);
            behaviour.on_start(&mut commands, asset_server);
        }
    }
}

pub fn clickable_click<T: ClickableBehaviour + Component>(
    mut commands: Commands,
    mouse_buttons: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<InGameState>>,
    keyboard_buttons: Res<Input<KeyCode>>,
    mut clickable: Query<(Entity, &mut T), With<Clicked>>,
) {
    if (!clickable.is_empty()) {
        let (entity, mut behaviour) = clickable.get_single_mut().unwrap();
        if (mouse_buttons.just_pressed(MouseButton::Left)) {
            behaviour.on_click(&mut commands, asset_server)
        }
        if (keyboard_buttons.just_pressed(KeyCode::Escape)) {
            game_state.set(InGameState::InGame);
            commands.entity(entity).remove::<Clicked>();
            behaviour.on_close(&mut commands)
        }
    }
}
