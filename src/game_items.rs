use bevy::utils::HashMap;
use crate::game::items::combination_lock::CombinationLock;
use crate::game::items::egyptian_necklace::components::EgyptianNecklace;
use crate::game::items::lecture_poster::component::LecturePoster;
use crate::game::items::librarydoor::components::LibraryDoor;
use crate::game::items::monitoring_room_door::MonitoringRoomDoor;
use crate::level_state::LevelState;
use crate::spawnable::components::SpawnableChild;

pub fn items_map() -> HashMap<LevelState, Vec<Box<dyn SpawnableChild + Sync + Send>>> {
    let mut map = HashMap::<LevelState, Vec<Box<dyn SpawnableChild + Sync + Send>>>::new();
    map.entry(LevelState::TrainPlatform)
        .or_insert(vec![Box::new(LibraryDoor), Box::new(EgyptianNecklace)]);
    map.entry(LevelState::TicketOffice).or_insert(vec![
        Box::new(LecturePoster),
        Box::new(MonitoringRoomDoor),
        Box::new(CombinationLock),
    ]);
    map
}
