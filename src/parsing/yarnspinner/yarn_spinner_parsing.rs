use std::fs;
use super::components::*;
use crate::pest::Parser;

#[derive(Parser)]
#[grammar = "assets/grammar/yarnspinner.pest"]
pub struct YarnSpinnerParser;

pub fn load_from_file(file_name: &str) -> Vec<Node> {
    let dialog = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let parsed = YarnSpinnerParser::parse(Rule::yarnspinner, &dialog).expect("unsuccessful parse");

    let mut nodes: Vec<Node> = vec![];
    for section in parsed.into_iter() {
        let mut node = Node {
            title: "".to_string(),
            lines: vec![],
        };
        match section.as_rule() {
            Rule::section => {
                for field in section.into_inner() {
                    match field.as_rule() {
                        Rule::title => {
                            node.title = field.as_str().to_string();
                        }
                        Rule::section_content => {
                            for content in field.into_inner() {
                                match content.as_rule() {
                                    Rule::set_line => {
                                        let mut variable_name = "".to_string();
                                        let mut value = false;
                                        for set_line_field in content.into_inner() {
                                            match set_line_field.as_rule() {
                                                Rule::variable_name => {
                                                    variable_name =
                                                        set_line_field.as_str().to_string();
                                                }
                                                Rule::boolean_value => {
                                                    value = set_line_field
                                                        .as_str()
                                                        .to_string()
                                                        .parse::<bool>()
                                                        .unwrap();
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        node.lines.push(LineType::SetLine {
                                            variable_name: variable_name,
                                            value: value,
                                        });
                                    }
                                    Rule::dialog_line => {
                                        let mut speaker = "".to_string();
                                        let mut text = "".to_string();
                                        let condition: Option<Condition> = None;
                                        for dialog_line_field in content.into_inner() {
                                            match dialog_line_field.as_rule() {
                                                Rule::speaker => {
                                                    speaker =
                                                        dialog_line_field.as_str().to_string();
                                                }
                                                Rule::dialog => {
                                                    text = dialog_line_field.as_str().to_string();
                                                }
                                                Rule::condition => {}
                                                _ => unreachable!(),
                                            }
                                        }
                                        node.lines.push(LineType::DialogLine {
                                            speaker: speaker,
                                            text: text,
                                            condition: condition,
                                        });
                                    }
                                    Rule::option_lines => {
                                        let mut option_possiblitites: Vec<OptionPossibility> =
                                            vec![];
                                        let speaker = "".to_string();
                                        for option_lines_field in content.into_inner() {
                                            match option_lines_field.as_rule() {
                                                Rule::option_line => {
                                                    let mut text = "".to_string();
                                                    let mut node_title = "".to_string();
                                                    let mut condition: Option<Condition> = None;
                                                    for option_line_field in
                                                        option_lines_field.into_inner()
                                                    {
                                                        match option_line_field.as_rule() {
                                                            Rule::dialog_line => {
                                                                let mut speaker = "".to_string();
                                                                for dialog_line_field in
                                                                    option_line_field.into_inner()
                                                                {
                                                                    match dialog_line_field
                                                                        .as_rule()
                                                                    {
                                                                        Rule::speaker => {
                                                                            speaker =
                                                                                dialog_line_field
                                                                                    .as_str()
                                                                                    .to_string();
                                                                        }
                                                                        Rule::dialog => {
                                                                            text =
                                                                                dialog_line_field
                                                                                    .as_str()
                                                                                    .to_string();
                                                                        }
                                                                        Rule::if_statement => {
                                                                            let mut variable_name =
                                                                                "".to_string();
                                                                            let mut condition_sign =
                                                                                "".to_string();
                                                                            let mut value =
                                                                                "".to_string();
                                                                            for if_statement_field in
                                                                                dialog_line_field
                                                                                    .into_inner()
                                                                            {
                                                                                match if_statement_field.as_rule() {
                                                                                    Rule::variable_name => {
                                                                                        variable_name = if_statement_field.as_str().to_string();
                                                                                    },
                                                                                    Rule::condition => {
                                                                                        condition_sign = if_statement_field.as_str().to_string();
                                                                                    },
                                                                                    Rule::boolean_value => {
                                                                                        value = if_statement_field.as_str().to_string();
                                                                                    },
                                                                                    _ => unreachable!()
                                                                                }
                                                                            }
                                                                            condition = Some(Condition { variable_name: variable_name, condition: condition_sign, value: value });
                                                                        }
                                                                        _ => unreachable!(),
                                                                    }
                                                                }
                                                            }
                                                            Rule::jump_line => {
                                                                for jump_line_field in
                                                                    option_line_field.into_inner()
                                                                {
                                                                    match jump_line_field.as_rule()
                                                                    {
                                                                        Rule::title => {
                                                                            node_title =
                                                                                jump_line_field
                                                                                    .as_str()
                                                                                    .to_string();
                                                                        }
                                                                        _ => unreachable!(),
                                                                    }
                                                                }
                                                            }
                                                            _ => unreachable!(),
                                                        }
                                                    }
                                                    option_possiblitites.push(OptionPossibility {
                                                        text: text,
                                                        jump_to_node: node_title,
                                                        condition: condition,
                                                    });
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        node.lines.push(LineType::OptionLine {
                                            speaker: speaker,
                                            possibilites: option_possiblitites,
                                        });
                                    }
                                    Rule::jump_line => {
                                        for jump_line_field in content.into_inner() {
                                            match jump_line_field.as_rule() {
                                                Rule::title => {
                                                    node.lines.push(LineType::JumpLine {
                                                        node_title: jump_line_field
                                                            .as_str()
                                                            .to_string(),
                                                    });
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                        Rule::position_number => {}
                        _ => unreachable!(),
                    }
                }
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
        nodes.push(node);
    }
    nodes
}
