use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::clickable::items::loverphoto::components::LoverPhoto;
use crate::game::effects::train_smoke::TrainSmoke;
use crate::level_state::LevelState;
use crate::spawnable::components::Spawnable;

pub fn items_map() -> HashMap<LevelState, Vec<Box<dyn Spawnable + Sync + Send>>> {
    let mut map = HashMap::<LevelState, Vec<Box<dyn Spawnable + Sync + Send>>>::new();
    map.entry(LevelState::TrainPlatform)
        .or_insert(vec![])
        .push(Box::new(LoverPhoto));
    map.entry(LevelState::TrainPlatform)
        .or_insert(vec![])
        .push(Box::new(TrainSmoke));
    map
}
