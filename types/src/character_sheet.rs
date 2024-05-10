use std::collections::HashMap;
use std::rc::Rc;

use crate::standard::limiters::NoLimiter;
use crate::standard::selectors::FirstSelector;

// todo: serde & serde_json

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CharacterSheetCollection {
    pub library: Library,
    pub types: Vec<TypeDefinition>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Library {
    pub functions: HashMap<String, FunctionDefinition>,
}

pub type FunctionArgs = Vec<StaticValueType>;

pub struct FunctionDefinition {
    pub name: String,
    pub function: Box<Rc<dyn Fn(FunctionArgs) -> Result<StaticValueType, CalculatorError>>>,
}

impl PartialEq for FunctionDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for FunctionDefinition {}

impl std::fmt::Debug for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionDefinition")
            .field("name", &self.name)
            .finish()
    }
}

impl Clone for FunctionDefinition {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            function: self.function.clone(),
        }
    }
}

pub struct CalculatorError {
    /// Message used for debugging, that may not be fit to be shown to the end user.
    pub internal_message: String,
    /// Message that may be shown to the end user. Should not use internal or technical information
    pub public_message: String,
}

impl CalculatorError {
    pub fn new(internal_message: String, public_message: String) -> Self {
        Self {
            internal_message,
            public_message,
        }
    }
    pub fn new_public(message: String) -> Self {
        Self {
            internal_message: message.clone(),
            public_message: message,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TypeDefinition {
    pub name: String,
    pub properties: Vec<PropertyDefinition>,
}

#[derive(Clone)]
pub struct PropertyDefinition {
    pub name: String,
    pub selector: Box<Rc<dyn Selector>>,
    pub limiter: Box<Rc<dyn Limiter>>,
}

impl PartialEq for PropertyDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for PropertyDefinition {}

impl Default for PropertyDefinition {
    fn default() -> Self {
        Self {
            name: String::default(),
            selector: Box::new(Rc::new(<dyn Selector>::default())),
            limiter: Box::new(Rc::new(<dyn Limiter>::default())),
        }
    }
}

impl std::fmt::Debug for PropertyDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PropertyDefinition")
            .field("name", &self.name)
            .field("selector", &self.selector.string_representation())
            .field("limiter", &self.limiter.string_representation())
            .finish()
    }
}

pub trait Selector: std::fmt::Debug {
    fn select(&self, items: Vec<StaticValueType>) -> StaticValueType;
    fn string_representation(&self) -> String;
}

impl dyn Selector {
    fn default() -> FirstSelector {
        FirstSelector {}
    }
}

pub trait Limiter {
    fn apply_limits(&self, value: &mut StaticValueType);
    fn string_representation(&self) -> String;
}

impl dyn Limiter {
    fn default() -> NoLimiter {
        NoLimiter {}
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Item {
    pub features: Vec<ItemFeature>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ItemFeature {
    pub name: String,
    pub description: String,
    pub base_type: String,
    pub modifiers: Vec<ItemFeatureModifier>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ItemFeatureModifier {
    pub referencing: Reference,
    pub value: CalculatedValue,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct Reference {
    pub parent: Option<Box<Reference>>,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CalculatedValue {
    StaticValue(StaticValueType),
    FunctionCall {
        function: String,
        arguments: FunctionArgs,
    },
}

impl Default for CalculatedValue {
    fn default() -> Self {
        Self::StaticValue(StaticValueType::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StaticValueType {
    Number(i32),
    Dice(DiceValue),
}

impl Default for StaticValueType {
    fn default() -> Self {
        Self::Number(0)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct DiceValue {
    pub dice: Vec<Dice>,
    pub bonus: i32,
}

impl DiceValue {
    pub fn add(&self, other: &StaticValueType) -> DiceValue {
        match other {
            StaticValueType::Number(n) => DiceValue {
                dice: self.dice.clone(),
                bonus: self.bonus + n,
            },
            StaticValueType::Dice(other_dice_set) => {
                let mut dice = self.dice.clone();
                'outer_loop: for other_dice in &other_dice_set.dice {
                    for curr_dice in &mut dice {
                        if curr_dice.sides == other_dice.sides
                            && curr_dice.modifiers == other_dice.modifiers
                        {
                            curr_dice.amount += other_dice.amount;
                            continue 'outer_loop;
                        }
                    }
                    dice.push(other_dice.clone());
                }
                DiceValue {
                    dice,
                    bonus: self.bonus + other_dice_set.bonus,
                }
            }
        }
    }

    pub fn minus(&self, other: &StaticValueType) -> DiceValue {
        match other {
            StaticValueType::Number(n) => DiceValue {
                dice: self.dice.clone(),
                bonus: self.bonus - n,
            },
            StaticValueType::Dice(other_dice_set) => {
                let mut dice = self.dice.clone();
                'outer_loop: for other_dice in &other_dice_set.dice {
                    for curr_dice in &mut dice {
                        if curr_dice.sides == other_dice.sides
                            && curr_dice.modifiers == other_dice.modifiers
                        {
                            curr_dice.amount -= other_dice.amount;
                            continue 'outer_loop;
                        }
                    }
                }
                dice.retain(|d| d.amount > 0);
                DiceValue {
                    dice,
                    bonus: self.bonus - other_dice_set.bonus,
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Dice {
    pub amount: u32,
    pub sides: u32,
    pub modifiers: Vec<DiceModifier>,
}

impl PartialOrd for Dice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self
            .sides
            .cmp(&other.sides)
            .then_with(|| self.amount.cmp(&other.amount))
            .then_with(|| self.modifiers.len().cmp(&other.modifiers.len()))
        {
            std::cmp::Ordering::Equal => {
                if self.eq(other) {
                    Some(std::cmp::Ordering::Equal)
                } else {
                    None
                }
            }
            o => Some(o),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiceModifier {
    Explode(DiceSelector),
    Keep(DiceSelector),
    Drop(DiceSelector),
    Reroll(DiceSelector),
    Count(DiceSelector),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiceSelector {
    /// highest x rolls of this dice set
    Highest(u16),
    /// lowest x rolls of this dice set
    Lowest(u16),
    /// all rolls this dice set with a roll higher than x
    HigherThan(u16),
    /// all rolls this dice set with a roll lower than x
    LowerThan(u16),
    /// all rolls this dice set with a roll equal to x
    Exactly(u16),
    /// all rolled dice (useful for e.g. count modifier)
    All,
}
