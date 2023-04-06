#[derive(Clone, Debug)]
pub struct Condition {
    pub variable_name: String,
    pub condition: String,
    pub value: String
}

#[derive(Clone, Debug)]
pub struct OptionPossibility {
    pub text: String,
    pub jump_to_node: String,
    pub condition: Option<Condition>
}

#[derive(Clone, Debug)]
pub enum LineType {
    SetLine { variable_name: String, value: bool },
    DialogLine { speaker: String, text: String, condition: Option<Condition> },
    JumpLine { node_title: String },
    OptionLine { speaker: String, possibilites: Vec<OptionPossibility> },
}

#[derive(Clone, Debug)]
pub struct Node {
    pub title: String,
    pub lines: Vec<LineType>,
}