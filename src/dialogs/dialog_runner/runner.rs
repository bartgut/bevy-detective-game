use crate::dialogs::dialog_runner::context::StateContext;
use super::components::*;
use crate::parsing::yarnspinner::components::*;
use bevy::utils::HashMap;

pub struct DialogRunner<T: StateContext + Default> {
    nodes: Vec<Node>,
    current_node: Node,
    current_line: LineType,
    current_line_index: usize,
    dialog_state: DialogState,
    context: T,
}

impl<T: StateContext + Default> DialogRunner<T> {
    pub fn create_from_nodes(nodes: Vec<Node>, start_node: &str) -> Self {
        let current_node = nodes
            .iter()
            .find(|node| node.title == start_node)
            .unwrap()
            .clone();
        let current_line = current_node.lines.first().unwrap().clone();
        Self {
            nodes,
            current_node,
            current_line,
            current_line_index: 0,
            dialog_state: DialogState::Start,
            context: T::default(),
        }
    }

    fn line_to_event(&self, line: &LineType) -> Option<DialogEvent> {
        match line {
            LineType::DialogLine {
                speaker,
                text,
                condition,
            } => Some(DialogEvent::Dialog {
                speaker: speaker.clone(),
                text: text.clone(),
            }),
            LineType::OptionLine {
                speaker,
                possibilites,
            } => {
                let options = possibilites
                    .iter()
                    .filter(|&x| self.passes_condition(x))
                    .map(|possibility| (possibility.text.clone(), possibility.jump_to_node.clone()))
                    .collect();
                Some(DialogEvent::Options {
                    speaker: speaker.clone(),
                    options,
                })
            }
            _ => None,
        }
    }

    fn passes_condition(&self, possibility: &OptionPossibility) -> bool {
        match &possibility.condition {
            Some(condition) => match self.context.get_value(&condition.variable_name) {
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

    fn update_context(&mut self) {
        match &self.current_line {
            LineType::SetLine {
                variable_name,
                value,
            } => {
                self.context.set_value(variable_name, value);
            }
            _ => {}
        }
    }

    fn perform_jump(&mut self) {
        match &self.current_line {
            LineType::JumpLine { node_title } => {
                self.current_node = self
                    .nodes
                    .iter()
                    .find(|node| node.title == *node_title)
                    .unwrap()
                    .clone();
                self.current_line = self.current_node.lines.first().unwrap().clone();
                self.current_line_index = 0;
            }
            _ => {}
        }
    }

    fn move_pointer(&mut self) {
        match self.dialog_state {
            DialogState::Waiting => {}
            _ => {
                self.current_line_index += 1;
                if self.current_line_index >= self.current_node.lines.len() {
                    self.dialog_state = DialogState::End;
                } else {
                    self.current_line = self.current_node.lines[self.current_line_index].clone();
                }
            }
        }
    }

    fn event_to_dialog_state(event: &Option<DialogEvent>) -> DialogState {
        match event {
            Some(DialogEvent::Dialog { speaker, text }) => DialogState::Dialog,
            Some(DialogEvent::Options { speaker, options }) => DialogState::Waiting,
            Some(DialogEvent::Waiting) => DialogState::Waiting,
            Some(DialogEvent::End) => DialogState::End,
            None => DialogState::Start, // TODO INNY StATE?
        }
    }

    pub fn next_event(&mut self) -> DialogEvent {
        match self.dialog_state {
            DialogState::Start => {
                self.update_context();
                self.perform_jump();
                let event = self.line_to_event(&self.current_line);
                let new_state = Self::event_to_dialog_state(&event);
                self.dialog_state = new_state.clone();
                self.move_pointer();
                match new_state {
                    DialogState::Start => self.next_event(),
                    _ => event.unwrap(),
                }
            }
            DialogState::Dialog => {
                self.update_context();
                self.perform_jump();
                let event = self.line_to_event(&self.current_line);
                let new_state = Self::event_to_dialog_state(&event);
                self.dialog_state = new_state.clone();
                self.move_pointer();
                match new_state {
                    DialogState::Start => self.next_event(),
                    _ => event.unwrap(),
                }
            }
            DialogState::Waiting => DialogEvent::Waiting,
            DialogState::End => DialogEvent::End,
            _ => DialogEvent::End,
        }
    }

    pub fn make_decision(&mut self, decision: String) {
        match self.dialog_state {
            DialogState::Waiting => {
                self.current_line = LineType::JumpLine {
                    node_title: decision.to_string(),
                };
                self.perform_jump();
                self.dialog_state = DialogState::Start;
            }
            _ => {}
        }
    }

    pub fn reset_to(&mut self, node_title: &str) {
        self.current_node = self
            .nodes
            .iter()
            .find(|node| node.title == node_title)
            .unwrap()
            .clone();
        self.current_line = self.current_node.lines.first().unwrap().clone();
        self.current_line_index = 0;
        self.dialog_state = DialogState::Start;
    }
}
