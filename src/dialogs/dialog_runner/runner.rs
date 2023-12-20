use std::marker::PhantomData;
use lazy_static::lazy_static;
use crate::dialogs::dialog_runner::context::StateContext;
use super::components::*;
use std::collections::HashMap;
use std::sync::Mutex;
use bevy::prelude::Commands;
use bevy_detective_derive::yarn_command;
use bevy_yarnspinner::parsing::components::{LineType, OptionPossibility, YarnSpinnerNode};
use crate::clickable::items::components::Collectible;
use crate::global_state::global_state::AddCollectibleToInventory;

type CommandFn = Box<dyn Fn(&mut Commands, &mut dyn Iterator<Item = String>) + Send + Sync>;
lazy_static! {
    static ref COMMAND_REGISTRY: Mutex<HashMap<String, CommandFn>> = Mutex::new(HashMap::new());
}

pub struct DialogRunner<T: StateContext> {
    nodes: Vec<YarnSpinnerNode>,
    current_node_index: usize,
    current_line_index: usize,
    dialog_state: DialogState,
    _phantom: PhantomData<T>,
}

impl<T: StateContext> DialogRunner<T> {
    pub fn current_node(&self) -> &YarnSpinnerNode {
        self.nodes
            .get(self.current_node_index)
            .expect("No current node")
    }

    pub fn current_line(&self) -> &LineType {
        let current_line = self.current_line_index;
        let current_node_lines = &self.current_node().lines;
        &current_node_lines[current_line]
    }

    pub fn current_node_mut(&mut self) -> &mut YarnSpinnerNode {
        self.nodes
            .get_mut(self.current_node_index)
            .expect("No current node")
    }

    pub fn current_line_mut(&mut self) -> &mut LineType {
        let current_line = self.current_line_index;
        let current_node_lines = &mut self.current_node_mut().lines;
        &mut current_node_lines[current_line]
    }

    pub fn register(&self) {
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

    pub fn create_from_nodes(nodes: Vec<YarnSpinnerNode>, start_node: &str) -> Self {
        let current_node_index = nodes
            .iter()
            .position(|node| node.title == start_node)
            .expect("No start node found");
        let x = Self {
            nodes,
            current_node_index: current_node_index,
            current_line_index: 0,
            dialog_state: DialogState::Start,
            _phantom: PhantomData,
        };
        x.register();
        x
    }

    fn line_to_event(&self, line: &LineType, context: &T) -> Option<DialogEvent> {
        match line {
            LineType::DialogLine {
                speaker,
                text,
                tags,
            } => Some(DialogEvent::Dialog {
                speaker: speaker.clone(),
                text: text.clone(),
                tags: tags.clone(),
            }),
            LineType::OptionLine {
                speaker,
                possibilities,
            } => {
                let options = possibilities
                    .iter()
                    .filter(|&x| self.passes_condition(x, context))
                    .map(|possibility| DialogOption {
                        text: possibility.text.clone(),
                        node: possibility.jump_to_node.clone(),
                        used: possibility.used.clone(),
                    })
                    .collect();
                Some(DialogEvent::Options {
                    speaker: speaker.clone(),
                    options,
                })
            }
            _ => None,
        }
    }

    fn passes_condition(&self, possibility: &OptionPossibility, context: &T) -> bool {
        match &possibility.condition {
            Some(condition) => match context.get_value(&condition.variable_name) {
                Some(value) => match condition.condition.as_str() {
                    "==" => value == &condition.value.parse::<bool>().unwrap(),
                    "!=" => value != &condition.value.parse::<bool>().unwrap(),
                    _ => false,
                },
                None => false,
            },
            None => true,
        }
    }

    fn update_context(&mut self, context: &mut T) {
        match &self.current_line() {
            LineType::SetLine {
                variable_name,
                value,
            } => {
                context.set_value(variable_name, value);
            }
            _ => {}
        }
    }

    fn perform_jump(&mut self) {
        match &self.current_line() {
            LineType::JumpLine { node_title } => {
                self.current_node_index = self
                    .nodes
                    .iter()
                    .position(|node| node.title == *node_title)
                    .expect("No node found");
                self.current_line_index = 0;
            }
            _ => {}
        }
    }

    fn execute_command(&mut self, commands: &mut Commands) {
        match &self.current_line() {
            LineType::CommandLine { func_name, args } => {
                COMMAND_REGISTRY.lock().unwrap().get(func_name).unwrap()(
                    commands,
                    &mut args.clone().into_iter(),
                );
            }
            _ => {}
        }
    }

    fn move_pointer(&mut self) {
        match self.dialog_state {
            DialogState::Waiting => {}
            _ => {
                self.current_line_index += 1;
                if self.current_line_index >= self.current_node().lines.len() {
                    self.dialog_state = DialogState::End;
                }
            }
        }
    }

    fn event_to_dialog_state(event: &Option<DialogEvent>) -> DialogState {
        match event {
            Some(DialogEvent::Dialog {
                speaker: _speaker,
                text: _text,
                tags: _tags,
            }) => DialogState::Dialog,
            Some(DialogEvent::Options {
                speaker: _speaker,
                options: _options,
            }) => DialogState::Waiting,
            Some(DialogEvent::Waiting) => DialogState::Waiting,
            Some(DialogEvent::End) => DialogState::End,
            None => DialogState::Start, // TODO INNY StATE?
        }
    }

    pub fn next_event(&mut self, context: &mut T, commands: &mut Commands) -> DialogEvent {
        match self.dialog_state {
            DialogState::Start => {
                self.update_context(context);
                self.perform_jump();
                self.execute_command(commands);
                let event = self.line_to_event(&self.current_line(), context);
                let new_state = Self::event_to_dialog_state(&event);
                self.dialog_state = new_state.clone();
                self.move_pointer();
                match new_state {
                    DialogState::Start => self.next_event(context, commands),
                    _ => event.unwrap(),
                }
            }
            DialogState::Dialog => {
                self.update_context(context);
                self.perform_jump();
                self.execute_command(commands);
                let event = self.line_to_event(&self.current_line(), context);
                let new_state = Self::event_to_dialog_state(&event);
                self.dialog_state = new_state.clone();
                self.move_pointer();
                match new_state {
                    DialogState::Start => self.next_event(context, commands),
                    _ => event.unwrap(),
                }
            }
            DialogState::Waiting => DialogEvent::Waiting,
            DialogState::End => DialogEvent::End,
        }
    }

    pub fn make_decision(&mut self, decision: String) {
        match self.dialog_state {
            DialogState::Waiting => {
                // set used to true
                match self.current_line_mut() {
                    LineType::OptionLine {
                        speaker: _speaker,
                        possibilities,
                    } => {
                        let mut possibility = possibilities
                            .iter_mut()
                            .find(|possibility| possibility.jump_to_node == decision)
                            .expect("No possibility found");
                        possibility.used = true;
                    }
                    _ => {}
                }

                self.current_node_index = self
                    .nodes
                    .iter()
                    .position(|node| node.title == decision)
                    .expect("No node found");
                self.current_line_index = 0;
                self.dialog_state = DialogState::Start;
            }
            _ => {}
        }
    }

    pub fn reset_to(&mut self, node_title: &str) {
        self.current_node_index = self
            .nodes
            .iter()
            .position(|node| node.title == node_title)
            .expect("No node found");
        self.current_line_index = 0;
        self.dialog_state = DialogState::Start;
    }
}
