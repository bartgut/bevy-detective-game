use bevy::utils::HashMap;
use crate::game::effects::train_smoke::TrainSmoke;
use crate::game::items::carinpark::components::CarInPark;
use crate::game::items::egyptian_necklace::components::EgyptianNecklace;
use crate::game::items::lecture_poster::component::LecturePoster;
use crate::game::items::librarydoor::components::LibraryDoor;
use crate::game::items::librarykeys::components::LibraryKeys;
use crate::game::items::loverphoto::components::LoverPhoto;
use crate::level_state::LevelState;
use crate::spawnable::components::SpawnableChild;

pub fn items_map() -> HashMap<LevelState, Vec<Box<dyn SpawnableChild + Sync + Send>>> {
    let mut map = HashMap::<LevelState, Vec<Box<dyn SpawnableChild + Sync + Send>>>::new();
    map.entry(LevelState::TrainPlatform).or_insert(vec![
        //Box::new(LoverPhoto),
        //Box::new(TrainSmoke),
        Box::new(LibraryDoor),
        Box::new(EgyptianNecklace),
        //Box::new(LecturePoster),
        //Box::new(LibraryKeys),
    ]);
    map.entry(LevelState::CityPark)
        .or_insert(vec![Box::new(CarInPark)]);
    map
}
