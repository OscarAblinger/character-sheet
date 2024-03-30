use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct CAST {
    pub nodes: Vec<Rc<RefCell<CASTNode>>>,
    pub model: CASTModel,
    pub references: Vec<Rc<RefCell<CASTReference>>>,
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct CASTReference {
    pub name: String
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct CASTNode {
    pub start_offset: i32,
    pub end_offset: i32,
    pub fmt_type: CASTNodeFmtType,
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum CASTNodeFmtType {
    Text(String),
    Whitespace(WhitespaceOptions),
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct WhitespaceOptions {
    pub spaces: i8,

    pub newlines: i8,
    pub max_newlines: i8,
    pub min_newlines: i8,

    pub indent_incr: i8,
    pub indent_decr: i8,
}

impl WhitespaceOptions {
    pub fn new_spaces(spaces: i8, newlines: i8) -> Self {
        WhitespaceOptions {
            spaces,
            newlines,
            max_newlines: 0,
            min_newlines: 0,
            indent_incr: 0,
            indent_decr: 0,
        }
    }
    pub fn new_no_ws() -> Self {
        WhitespaceOptions {
            spaces: 0,
            newlines: 0,
            max_newlines: 0,
            min_newlines: 0,
            indent_incr: 0,
            indent_decr: 0,
        }
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct NodeReferenceString {
    pub node: Rc<RefCell<CASTNode>>,
}

impl NodeReferenceString {
    pub fn new(node: Rc<RefCell<CASTNode>>) -> Self {
        NodeReferenceString { node }
    }

    pub fn get(&self) -> String {
        match &self.node.borrow().fmt_type {
            CASTNodeFmtType::Text(str) => str.clone(),
            CASTNodeFmtType::Whitespace(_) => panic!("Expected text node, got whitespace node"),
        }
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct NodeReferenceI32 {
    pub node: Rc<RefCell<CASTNode>>,
}

impl NodeReferenceI32 {
    pub fn new(node: Rc<RefCell<CASTNode>>) -> Self {
        NodeReferenceI32 { node }
    }
    
    pub fn get(&self) -> i32 {
        match &self.node.borrow().fmt_type {
            CASTNodeFmtType::Text(str) => str.parse().unwrap(),
            CASTNodeFmtType::Whitespace(_) => panic!("Expected text node, got whitespace node"),
        }
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct CASTModel {
    pub features: Vec<CASTFeature>,
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct CASTFeature {
    pub name: NodeReferenceString,
    pub description: NodeReferenceString,
    pub modifiers: Vec<CASTModifier>,
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct CASTModifier {
    pub referencing: Rc<RefCell<CASTReference>>,
    pub value: CASTModifierValue,
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum CASTModifierValue {
    SimpleBonus(NodeReferenceI32),
    Bonus(NodeReferenceI32),
    Set(NodeReferenceI32),
}
