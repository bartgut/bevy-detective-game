use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::quests::components::QuestStatus;

#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct QuestRaw {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: QuestStatus,
    pub activation_condition: Vec<String>,
    pub completion_condition: Vec<String>,
    pub subsequent_tasks: Vec<String>,
}

#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct QuestBundle {
    pub quests: Vec<QuestRaw>,
}

// events
#[derive(Event)]
pub enum QuestEventMessage {
    NewQuest(String),
    QuestCompleted(String),
}
