use crate::parser::ast::*;

pub fn serialize(ast: &AST) -> String {
    let serialize_nodes = create_serialize_nodes(&ast);

    print_nodes(serialize_nodes)
}

fn print_nodes(serialize_nodes: Vec<SerializeNode>) -> String {
    let mut output: String = String::new();
    let mut curr_indent = 0;
    for node in serialize_nodes {
        match node {
            SerializeNode::Text(t) => {
                output.push_str(&t);
            },
            SerializeNode::Whitespace(ws) => {
                curr_indent += ws.indent_incr - ws.indent_decr;
                curr_indent = curr_indent.max(0);

                let nl = ws.newline.min(ws.newline_max).max(ws.newline_min);
                if nl > 0 {
                    output.push_str(&("\n".repeat(nl.try_into().unwrap())));
                    output.push_str(&("  ".repeat(curr_indent.try_into().unwrap())));
                } else {
                    output.push_str(&(" ".repeat(ws.spaces.max(0).try_into().unwrap())));
                }
            },
        }
    }
    output
}

#[derive(Debug)]
pub enum SerializeNode {
    Whitespace(WhitespaceOptions),
    Text(String),
}

impl SerializeNode {
    pub fn new_no_space() -> Self {
        Self::Whitespace(WhitespaceOptions::new_no_space())
    }
    pub fn new_space() -> Self {
        Self::Whitespace(WhitespaceOptions::new_space())
    }
    pub fn new_newline() -> Self {
        Self::Whitespace(WhitespaceOptions::new_newline())
    }
    pub fn new_text(text: &str) -> Self {
        Self::Text(text.to_string())
    }
}

#[derive(Debug)]
pub struct WhitespaceOptions {
    pub indent_incr: i8,
    pub indent_decr: i8,

    pub newline_max: i8,
    pub newline_min: i8,
    pub newline: i8,

    pub spaces: i8,
}

impl WhitespaceOptions {
    pub fn new_no_space() -> Self {
        WhitespaceOptions {
            indent_incr: 0,
            indent_decr: 0,

            newline_max: 0,
            newline_min: 0,
            newline: 0,

            spaces: 0,
        }
    }
    pub fn new_space() -> Self {
        WhitespaceOptions {
            indent_incr: 0,
            indent_decr: 0,

            newline_max: 0,
            newline_min: 0,
            newline: 0,

            spaces: 1,
        }
    }
    pub fn new_newline() -> Self {
        WhitespaceOptions {
            indent_incr: 0,
            indent_decr: 0,

            newline_max: 2,
            newline_min: 1,
            newline: 1,

            spaces: 0,
        }
    }

    pub fn with_indent_incr(mut self, i: i8) -> Self {
        self.indent_incr = self.indent_incr + i;
        self
    }
}

fn create_serialize_nodes(ast: &AST) -> Vec<SerializeNode> {
    let nodes = Vec::new();
    let mut serializer = Serializer { nodes };

    serializer.serialize_model(&ast.model);

    serializer.nodes
}

#[derive(Debug)]
struct Serializer {
    pub nodes: Vec<SerializeNode>,
}

impl Serializer {
    fn decrease_indent(&mut self, nr: i8) {
        match self.nodes.split_last_mut() {
            Some((SerializeNode::Whitespace(ws), _)) => {
                ws.indent_decr = ws.indent_decr + nr;
            },
            Some((_, nodes)) => {
                match nodes.last_mut() {
                    Some(SerializeNode::Whitespace(ws)) => {
                        ws.indent_decr = ws.indent_decr + nr;
                    },
                    Some(_) => {
                        panic!("invalid node structure: {:?}", self);
                    }
                    None => {},
                }
            },
            None => {},
        }
    }

    fn serialize_model(&mut self, model: &Model) {
        self.nodes.push(SerializeNode::new_no_space());

        for field in &model.features {
            self.serialize_feature(field);
        }
    }

    fn serialize_feature(&mut self, feature: &Feature) {
        self.nodes.push(SerializeNode::new_text("Name"));
        self.nodes.push(SerializeNode::new_no_space());
        self.nodes.push(SerializeNode::new_text(":"));
        self.nodes.push(SerializeNode::new_space());
        self.nodes.push(SerializeNode::new_text(&("\"".to_string() + &feature.name + "\"")));

        self.nodes.push(SerializeNode::new_newline());

        self.nodes.push(SerializeNode::new_text("Description"));
        self.nodes.push(SerializeNode::new_no_space());
        self.nodes.push(SerializeNode::new_text(":"));
        self.nodes.push(SerializeNode::new_space());
        self.nodes.push(SerializeNode::new_text(&("\"".to_string() + &feature.description + "\"")));
        self.nodes.push(SerializeNode::new_newline());

        if feature.modifiers.len() > 0 {
            self.nodes.push(SerializeNode::new_text("Modifiers"));
            self.nodes.push(SerializeNode::new_no_space());
            self.nodes.push(SerializeNode::new_text(":"));
            self.nodes.push(SerializeNode::Whitespace(WhitespaceOptions::new_newline().with_indent_incr(1)));

            for modifier in &feature.modifiers {
                self.serialize_modifier(modifier);
            }
            self.decrease_indent(1);
        }
    }

    fn serialize_modifier(&mut self, modifier: &Modifier) {
        match modifier.value {
            ModifierValue::SimpleBonus(v) => {
                if v >= 0 {
                    self.nodes.push(SerializeNode::new_text(&("+".to_string() + &v.to_string())));
                } else {
                    self.nodes.push(SerializeNode::new_text(&v.to_string()));
                }
                self.nodes.push(SerializeNode::new_space());
                self.nodes.push(SerializeNode::new_text(&modifier.referencing.name));
                self.nodes.push(SerializeNode::new_no_space());
                self.nodes.push(SerializeNode::new_text(";"));
                self.nodes.push(SerializeNode::new_newline());
            },
            ModifierValue::Bonus(v) => {
                self.nodes.push(SerializeNode::new_text("bonus"));
                self.nodes.push(SerializeNode::new_space());
                self.nodes.push(SerializeNode::new_text("to"));
                self.nodes.push(SerializeNode::new_space());
                self.nodes.push(SerializeNode::new_text(&modifier.referencing.name));
                self.nodes.push(SerializeNode::new_space());
                self.nodes.push(SerializeNode::new_text("of"));
                self.nodes.push(SerializeNode::new_space());
                self.nodes.push(SerializeNode::new_text(&(v.to_string())));
                self.nodes.push(SerializeNode::new_no_space());
                self.nodes.push(SerializeNode::new_text(";"));
                self.nodes.push(SerializeNode::new_newline());
            },
            ModifierValue::Set(v) => {
                self.nodes.push(SerializeNode::new_text("set"));
                self.nodes.push(SerializeNode::new_space());
                self.nodes.push(SerializeNode::new_text(&modifier.referencing.name));
                self.nodes.push(SerializeNode::new_space());
                self.nodes.push(SerializeNode::new_text("to"));
                self.nodes.push(SerializeNode::new_space());
                self.nodes.push(SerializeNode::new_text(&(v.to_string())));
                self.nodes.push(SerializeNode::new_no_space());
                self.nodes.push(SerializeNode::new_text(";"));
                self.nodes.push(SerializeNode::new_newline());
            }
        }
    }
}
