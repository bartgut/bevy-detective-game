use bevy::prelude::*;
use crate::clickable::items::behaviour::ClickableBehaviour;
use crate::in_game_state::InGameState;
use crate::level_state::LevelState;

#[derive(Component)]
pub struct LevelDescription {
    pub level_name: String,
    pub level_initial_position: Vec3,
    pub player_initial_position: Vec3,
}

#[derive(Component)]
pub struct CurrentLevel;

#[derive(Component)]
pub struct CurrentLevelSprite;

#[derive(Component)]
pub struct LevelChangeTrigger {
    pub level_state: LevelState,
}

#[derive(Component)]
pub struct LevelTeleport {
    pub level_state: LevelState,
}

impl ClickableBehaviour for LevelTeleport {
    fn on_start(&mut self, commands: &mut Commands, _: Res<AssetServer>) {
        commands.spawn(LevelChangeTrigger {
            level_state: self.level_state.clone(),
        });
    }

    fn on_click(&mut self, _: &mut Commands, _: Res<AssetServer>) {
        todo!()
    }

    fn on_close(&mut self, _: &mut Commands) {
        todo!()
    }
}
