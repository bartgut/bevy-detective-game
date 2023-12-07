use bevy::prelude::*;

#[derive(Component)]
pub struct EventHandler<T: Event> {
    pub event_handle: fn(&mut Commands, &T),
}
