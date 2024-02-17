pub struct AST {
    model: Model,
    references: Vec<Reference>,
}

pub enum Scope {
    Character, // essentially global scope
    Feature(String), // for referencing properties
}

pub struct Reference {
    scope: Scope,
    name: String,
}

pub struct Model {
    features: Vec<Feature>,
}

pub struct Feature {
    name: String,
    description: String,
    modifiers: Vec<Modifier>,
}

pub struct Modifier {
    referencing: Reference, // todo: ownership?
}

