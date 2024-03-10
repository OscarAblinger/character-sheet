pub trait CSTNode {
    fn get_offset(&self) -> i32;
    fn get_end_offset(&self) -> i32;
    fn get_children(&self) -> Vec<&dyn CSTNode>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CST {
    pub model: CSTModel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CSTModel {
    pub offset: i32,
    pub end_offset: i32,
    pub features: Vec<CSTFeature>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CSTFeature {
    pub offset: i32,
    pub end_offset: i32,
    pub name: String,
    pub description: String,
    pub modifiers: Vec<CSTModifier>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CSTModifier {
    pub offset: i32,
    pub end_offset: i32,
    pub reference: String,
    pub value: CSTModifierValue,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CSTModifierValue {
    Bonus{offset: i32, end_offset: i32, value: i32 },
    Set{offset: i32, end_offset: i32, value: i32 },
}


impl CSTNode for CSTModel {
    fn get_offset(&self) -> i32 {
        self.offset
    }
    fn get_children(&self) -> Vec<&dyn CSTNode> {
        self.features
    }
}

impl CSTNode for CSTFeature {
    fn get_offset(&self) -> i32 {
        self.offset
    }
    fn get_children(&self) -> Vec<&dyn CSTNode> {
        self.modifiers
    }
}

impl CSTNode for CSTModifier {
    fn get_offset(&self) -> i32 {
        self.offset
    }
    fn get_children(&self) -> Vec<&dyn CSTNode> {
        vec![&self.value]
    }
}

impl CSTNode for CSTModifierValue {
    fn get_offset(&self) -> i32 {
        match self {
            CSTModifierValue::Bonus{offset, ..} => *offset,
            CSTModifierValue::Set{offset, ..} => *offset,
        }
    }
    fn get_children(&self) -> Vec<&dyn CSTNode> {
        vec![]
    }
}
