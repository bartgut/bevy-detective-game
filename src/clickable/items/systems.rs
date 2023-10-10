use bevy::prelude::*;
use crate::clickable::components::Clicked;
use crate::clickable::items::components::Collectible;

pub fn collectible_click(
    mut commands: Commands,
    mut removed_components: RemovedComponents<Clicked>,
    mut collectible: Query<(Entity, &mut Collectible)>,
) {
    if !collectible.is_empty() {
        let (entity_collectible, behaviour) = collectible.get_single_mut().unwrap();
        for entity in removed_components.iter() {
            if entity_collectible == entity {
                behaviour.on_collect(&mut commands);
                commands.entity(entity_collectible).despawn_recursive();
            }
        }
    }
}
