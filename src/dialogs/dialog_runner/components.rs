#[derive(Clone, Debug)]
pub enum DialogState {
    Start,
    Dialog,
    Waiting,
    Ready,
    End
}

#[derive(Clone, Debug)]
pub enum DialogEvent {
    Dialog { speaker: String, text: String },
    Options { speaker: String, options: Vec<(String, String)>},
    Waiting,
    End
}