use bevy::prelude::*;
use bevy_yarnspinner::bevy_detective_derive::yarn_command;
use crate::clickable::items::components::Collectible;
use crate::global_state::global_state::AddCollectibleToInventory;
use bevy_yarnspinner::dialog_runner::runner::COMMAND_REGISTRY;

pub fn yarn_commands_register() {
    #[yarn_command("addInventory")]
    fn func_commands(commands: &mut Commands, item_name: String, item_description: String) {
        commands.spawn(AddCollectibleToInventory {
            0: Collectible {
                inventory_sprite: "test.png".to_string(),
                name: item_name,
                description: item_description,
            },
        });
    }
}
