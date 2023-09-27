use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::math::Vec3Swizzles;
use crate::clickable::components::{CanBeClicked, Clickable, Clicked, HoveredOverClickable};
use crate::clickable::items::behaviour::ClickableBehaviour;
use crate::clickable::items::resource::ItemResource;
use crate::game::world_map::world_map::WorldMap;
use crate::in_game_state::InGameState;
use crate::level_state::LevelState;
use crate::levels::components::CurrentLevelSprite;
use crate::player::components::Player;

pub fn gray_out_all(
    mut level_query: Query<(&mut Sprite, &Children), With<CurrentLevelSprite>>,
    mut sprites: Query<&mut Sprite, Without<CurrentLevelSprite>>,
) {
    let (mut level_sprite, children) = level_query.get_single_mut().unwrap();
    level_sprite.color = Color::rgb(0.4, 0.4, 0.4);
    for &child in children.iter() {
        let sprite = sprites.get_mut(child);
        match sprite {
            Ok(mut existing) => {
                existing.color = Color::rgb(0.4, 0.4, 0.4);
            }
            Err(_) => {}
        }
    }
}

pub fn return_to_normal_colors(
    mut level_query: Query<(&mut Sprite, &Children), With<CurrentLevelSprite>>,
    mut sprites: Query<&mut Sprite, Without<CurrentLevelSprite>>,
) {
    for (mut level_sprite, children) in level_query.iter_mut() {
        level_sprite.color = Color::default();
        for &child in children.iter() {
            let sprite = sprites.get_mut(child);
            match sprite {
                Ok(mut existing) => existing.color = Color::default(),
                Err(_) => {}
            }
        }
    }
}

pub fn initialize_items(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_query: Query<Entity, With<CurrentLevelSprite>>,
    level_state: Res<State<LevelState>>,
    items_in_levels: Res<ItemResource>,
) {
    let level = level_query.get_single().unwrap();
    for item in items_in_levels
        .items
        .get(level_state.get())
        .unwrap_or(&vec![])
    {
        item.spawn_child(&mut commands.entity(level), &asset_server)
    }
}

pub fn print_when_hovered_clickable_global(
    mut commands: Commands,
    mut cursor_evr: EventReader<CursorMoved>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    map_query: Query<&mut Transform, (With<WorldMap>, Without<CanBeClicked>)>,
    clickable_query: Query<(Entity, &Transform), With<CanBeClicked>>,
    clickable_query_already_hovered: Query<(Entity, &HoveredOverClickable)>,
) {
    for map in map_query.iter() {
        let mut window = window_query.get_single_mut().unwrap();

        for ev in cursor_evr.iter() {
            for (entity, clickable_transform) in clickable_query.iter() {
                let global_position = ev.position.x - window.resolution.width() / 2.0;
                let clickable_position = clickable_transform.translation.xy() * map.scale.xy();
                if Vec2::new(global_position, 0.0).distance(Vec2::new(clickable_position.x, 0.0))
                    < 30.0
                {
                    window.cursor.icon = CursorIcon::Hand;
                    if !clickable_query_already_hovered.contains(entity) {
                        commands.entity(entity).insert(HoveredOverClickable);
                    }
                } else {
                    window.cursor.icon = CursorIcon::Default;
                    commands.entity(entity).remove::<HoveredOverClickable>();
                }
            }
        }
    }
}

pub fn print_when_hovered_clickable(
    mut commands: Commands,
    mut cursor_evr: EventReader<CursorMoved>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    level_query: Query<&Transform, (Without<Player>, With<CurrentLevelSprite>)>,
    clickable_query: Query<(Entity, &Transform), With<CanBeClicked>>,
    clickable_query_already_hovered: Query<(Entity, With<HoveredOverClickable>)>,
) {
    let width_halved = 2688.0 / 2.0; // TODO
    let mut window = window_query.get_single_mut().unwrap();
    for ev in cursor_evr.iter() {
        for (entity, clickable_transform) in clickable_query.iter() {
            for level_transform in level_query.iter() {
                let new_level_position = (width_halved - level_transform.translation.x)
                    + (ev.position.x - window.resolution.width() / 2.0);
                let npc_position_2d = width_halved
                    + clickable_transform.translation.xy() * level_transform.scale.xy();
                if Vec2::new(new_level_position, 0.0).distance(Vec2::new(npc_position_2d.x, 0.0))
                    < 30.0
                {
                    window.cursor.icon = CursorIcon::Hand;
                    if !clickable_query_already_hovered.contains(entity) {
                        commands.entity(entity).insert(HoveredOverClickable);
                    }
                } else {
                    window.cursor.icon = CursorIcon::Default;
                    commands.entity(entity).remove::<HoveredOverClickable>();
                }
            }
        }
    }
}

pub fn hover_entry<T: Component + ClickableBehaviour>(
    mut commands: Commands,
    mut query: Query<&mut T, Added<HoveredOverClickable>>,
) {
    for mut hovered in query.iter_mut() {
        hovered.on_hover_entry(&mut commands);
    }
}

pub fn clickable_can_be_clicked(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    clickable_query: Query<(Entity, &Transform, &Clickable)>,
) {
    for player_transform in player_query.iter() {
        for (entity, clickable_transform, clickable) in clickable_query.iter() {
            if player_transform
                .translation
                .distance(clickable_transform.translation)
                < clickable.required_distance
            {
                commands.entity(entity).insert(CanBeClicked);
            } else {
                commands.entity(entity).remove::<CanBeClicked>();
            }
        }
    }
}

pub fn clickable_first_click(
    mut commands: Commands,
    mut clickable: Query<Entity, With<HoveredOverClickable>>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if !clickable.is_empty() {
            let entity = clickable.get_single_mut().unwrap();
            commands
                .entity(entity)
                .insert(Clicked)
                .remove::<HoveredOverClickable>();
        }
    }
}

pub fn clickable_clicked<T: ClickableBehaviour + Component>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<InGameState>>,
    mut clickable: Query<&mut T, Added<Clicked>>,
) {
    match clickable.get_single_mut() {
        Ok(mut behaviour) => {
            game_state.set(InGameState::LookingAtItem);
            behaviour.on_start(&mut commands, asset_server);
        }
        Err(_) => {}
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
    if !clickable.is_empty() {
        let (entity, mut behaviour) = clickable.get_single_mut().unwrap();
        if mouse_buttons.just_pressed(MouseButton::Left) {
            behaviour.on_click(&mut commands, asset_server)
        }
        if keyboard_buttons.just_pressed(KeyCode::Escape) {
            game_state.set(InGameState::InGame);
            commands.entity(entity).remove::<Clicked>();
            behaviour.on_close(&mut commands)
        }
    }
}
