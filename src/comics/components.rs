use bevy::prelude::*;
use crate::comics::systems::ComicsSequence;

#[derive(Bundle)]
pub struct SinglePageComicsBundle<T: ComicsSequence + Component> {
    pub sequence: T,
}
