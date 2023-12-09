use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::quests::loader::format::QuestBundle;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum QuestStatus {
    Inactive,
    Active,
    Complete,
}

#[derive(Resource)]
pub struct QuestSystem {
    pub quest: Handle<QuestBundle>,
}
