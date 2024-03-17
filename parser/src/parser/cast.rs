use core::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CAST {
    pub model: CASTModel,
    pub references: Vec<Rc<RefCell<CASTReference>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualToken {
    pub offset: i32,
    pub end: i32, // offset + length (= offset of next token or end of file)
    pub format: Format,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Format {
    Text(Rc<RefCell<String>>),
    WhiteSpace(WsFormatInfo),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WsFormatInfo {
    pub spaces: i8,
    pub preferred_newlines: Option<i8>,
    pub min_newlines: Option<i8>,
    pub max_newlines: i8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CASTReference {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CASTModel {
    pub tokens: Vec<Rc<RefCell<VirtualToken>>>,
    pub features: Vec<CASTFeature>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CASTFeature {
    pub tokens: Vec<Rc<RefCell<VirtualToken>>>,
    pub name: Rc<RefCell<String>>,
    pub description: Rc<RefCell<String>>,
    pub modifiers: Vec<CASTModifier>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CASTModifier {
    pub tokens: Vec<Rc<RefCell<VirtualToken>>>,
    pub reference: Rc<RefCell<CASTReference>>,
    pub value: Rc<RefCell<i32>>,
    pub value_type: CASTModifierType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CASTModifierType {
    Bonus,
    Set,
}
