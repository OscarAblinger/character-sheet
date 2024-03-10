#[derive(Debug, Clone)]
pub struct AST {
    pub model: Model,
    pub references: Vec<Reference>,
}

#[derive(Debug, Clone)]
pub enum Scope {
    Character,       // essentially global scope
    Feature(String), // for referencing properties
}

#[derive(Debug, Clone)]
pub struct Reference {
    pub scope: Scope,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub features: Vec<Feature>,
}

#[derive(Debug, Clone)]
pub struct Feature {
    pub name: String,
    pub description: String,
    pub modifiers: Vec<Modifier>,
}

#[derive(Debug, Clone)]
pub struct Modifier {
    pub referencing: Reference,
    pub value: ModifierValue,
}

#[derive(Debug, Copy, Clone)]
pub enum ModifierValue {
    Bonus(i32),
    Set(i32),
}
