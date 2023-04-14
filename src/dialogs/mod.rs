mod dialog_runner;
mod dialogs;
mod ui;

use bevy::prelude::*;
use crate::dialogs::dialogs::resource::Dialogs;
use ui::systems::*;
use crate::game_state::GameState;
use crate::in_game_state::InGameState;

pub struct DialogsPlugin;

impl Plugin for DialogsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Dialogs::load_from_file(
            "assets/dialogs/first_dialog.yarn",
            "Librarian1PlayerConversationIntro",
        ))
        .add_system(interact_with_dialog_text.run_if(in_state(GameState::InGame)))
        .add_system(start_dialog.run_if(in_state(InGameState::InGame)))
        .add_system(
            mouse_button_input
                .after(interact_with_dialog_text)
                .run_if(in_state(InGameState::Dialog)),
        );
    }
}
