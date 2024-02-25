#[derive(Debug)]
pub struct AST {
    pub model: Model,
    pub references: Vec<Reference>,
}

#[derive(Debug)]
pub enum Scope {
    Character, // essentially global scope
    Feature(String), // for referencing properties
}

#[derive(Debug)]
pub struct Reference {
    pub scope: Scope,
    pub name: String,
}

#[derive(Debug)]
pub struct Model {
    pub features: Vec<Feature>,
}

#[derive(Debug)]
pub struct Feature {
    pub name: String,
    pub description: String,
    pub modifiers: Vec<Modifier>,
}

#[derive(Debug)]
pub struct Modifier {
    pub referencing: Reference, // todo: ownership?
    pub value: ModifierValue,
}

#[derive(Debug)]
pub enum ModifierValue {
    Bonus(i32),
    Set(i32),
}

