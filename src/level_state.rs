use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum LevelState {
    #[default]
    None,
    TrainPlatform,
    TicketOffice,
    CityPark,
    Hospital,
    LibraryInternals,
    Morgue,
}
