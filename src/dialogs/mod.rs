pub mod dialog_runner;
pub mod dialogs;
pub mod ui;

use bevy::prelude::*;
use ui::systems::*;
use crate::dialogs::ui::components::OptionUINode;
use crate::in_game_state::InGameState;
use crate::main_menu::components::MainMenuButton;

pub struct DialogsPlugin;

impl Plugin for DialogsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::Dialog), load_dialog)
            .add_systems(
                Update,
                interact_with_dialog_text.run_if(in_state(InGameState::Dialog)),
            )
            .add_systems(Update, start_dialog.run_if(in_state(InGameState::InGame)))
            .add_systems(
                Update,
                mouse_button_input
                    .after(interact_with_dialog_text)
                    .run_if(in_state(InGameState::Dialog)),
            )
            .add_systems(Update, dialog_ui_from_event)
            .add_systems(Update, dialog_ui_events_with_timer_ownership);
    }
}
