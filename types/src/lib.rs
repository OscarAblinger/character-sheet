use std::collections::HashMap;

pub struct StaticPropertyDefinition<'a, T> {
    pub description: Option<&'a str>,
    pub default_value: T,
    pub limiter: Box<dyn Fn(T) -> T>,
    pub selector: Box<dyn Fn(Vec<T>) -> T>,
}

pub struct CalculatedPropertyDefinition<'a> {
    pub description: Option<&'a str>,
}

pub enum PropertyDefinition<'a> {
    StaticInt(StaticPropertyDefinition<'a, i32>),
    StaticFloat(StaticPropertyDefinition<'a, f32>),
    StaticString(StaticPropertyDefinition<'a, &'a str>),
    Calculated(CalculatedPropertyDefinition<'a>),
}

pub struct CharacterDefinition<'definition> {
    pub fields: HashMap<String, PropertyDefinition<'definition>>,
}

pub enum ArithmeticMutation {
    Addition,
    Substraction,
    Multiplication,
    Division,
}

pub enum PropertyMutationValue<'a> {
    StaticInt(i32),
    StaticFloat(f32),
    StaticString(&'a str),

    StackableArithmeticMutation(ArithmeticMutation, f32),
}

pub struct PropertyMutation<'a> {
    pub description: Option<&'a str>,
    pub value: PropertyMutationValue<'a>,
}

pub struct CharacterAddon<'addon> {
    pub description: Option<&'addon str>,
    pub short_description: Option<&'addon str>,
    pub field_mutations: HashMap<String, PropertyMutation<'addon>>,
}
