use bevy::prelude::*;
use crate::clickable::components::Clicked;
use crate::clickable::items::components::Collectible;

pub fn collectible_click(
    mut commands: Commands,
    mut collectible: Query<(Entity, &mut Collectible), Added<Clicked>>,
) {
    if !collectible.is_empty() {
        let (_entity, behaviour) = collectible.get_single_mut().unwrap();
        behaviour.on_collect(&mut commands);
        //commands.entity(entity).remove::<Clicked>();
    }
}
