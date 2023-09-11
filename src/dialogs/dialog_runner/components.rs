use crate::parsing::yarnspinner::components::Tag;

#[derive(Clone, Debug)]
pub enum DialogState {
    Start,
    Dialog,
    Waiting,
    End,
}

#[derive(Clone, Debug)]
pub enum DialogEvent {
    Dialog {
        speaker: String,
        text: String,
        tags: Vec<Tag>,
    },
    Options {
        speaker: String,
        options: Vec<(String, String)>,
    },
    Waiting,
    End,
}
