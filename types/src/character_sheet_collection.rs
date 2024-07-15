#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A collection of types and items.
/// The "CS" prefix is refering to the name of the library not an actual character sheet.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase", deny_unknown_fields))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CSCollection {
    /// Type definitions of all properties.
    /// This may contain properties that are not in any feature, as well as miss some properties
    /// that are.
    /// Type definitions that are not referenced in any feature were simply explicitely specified.
    /// Type definitions that are not in this list, but referenced in items, were not specified, so
    /// they should be considered as the default value.
    pub properties: Vec<PropertyDefinition>,
    pub items: Vec<FeatureSet>,
}

impl CSCollection {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Definition of the type of a property
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase", deny_unknown_fields))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PropertyDefinition {
    /// Name of the property.
    pub name: String,
    /// In case multiple values for this property are possible, this selector specifies which ones
    /// should be kept.
    pub selector: Selector,
    /// The limiters limit the possible values of this property.
    pub limiters: Vec<Limiter>,
}

/// A selector selects a given value out of a list of possible ones.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase", deny_unknown_fields))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Selector {
    /// The identifier of the selector.
    /// Example: `highest`
    /// The list of supported selectors depends on the execution engine.
    pub identifier: String,
    /// Potential arguments for the selector.
    pub arguments: Vec<String>,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase", deny_unknown_fields))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Limiter {
    /// The identifier of the limiter.
    /// Example: `maximum`
    /// The list of supported limiters depends on the execution engine.
    pub identifier: String,
    /// Potential arguments for the limiter.
    pub arguments: Vec<String>,
}

/// As the name implies a feature set bundles a bunch of features together.
/// In most games this may be anything from classes to races to items or even spells in some cases.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase", deny_unknown_fields))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct FeatureSet {
    pub name: String,
    pub description: String,
    pub features: Vec<Feature>,
}

/// A feature is any actual value that a character may have.
/// This can range from things like HP or Mana all the way to Attacks and spells.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase", deny_unknown_fields))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Feature {
    pub name: String,
    pub description: String,
    /// The base types should mostly be specified by the base rules for the game system.
    /// They help you categorize it into the proper sections of your UI as well as use them in
    /// filters etc..
    pub base_type: String,
    /// The modifiers specify which values will be changed how, if this feature is active.
    pub modifiers: Vec<FeatureModifier>,
}

/// A set of changes to a given property.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase", deny_unknown_fields))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct FeatureModifier {
    /// The property of the character that this references.
    pub property: String,
    /// The changes applied to the property.
    pub value: CalculatedValue,
}

/// A set of operations that will result in a specific value.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase", deny_unknown_fields))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CalculatedValue {
    /// A static value always has the same value.
    StaticValue(StaticValueType),
    /// A script is a value that depends on some operations and usually other properties.
    Script(Script),
}

impl Default for CalculatedValue {
    fn default() -> Self {
        Self::StaticValue(StaticValueType::default())
    }
}

/// An actual value that a property may have.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase", deny_unknown_fields))]
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

/// A script is a value that depends on some operations and usually other properties.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase", deny_unknown_fields))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Script {
    // todo: specify the syntax of the script
    /// The script that specifies how the value should be calculated once all dependencies are
    /// calculated.
    /// May reference the dependencies, but never other properties outside of them.
    pub script: String,
    /// The list of properties that this script depends on.
    pub dependencies: Vec<String>,
}

/// A value that consists of a bunch of dice that should be rolled to get the actual value.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase", deny_unknown_fields))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct DiceValue {
    /// The dice sets to be rolled.
    pub dice: Vec<Dice>,
    /// A static bonus to the result of the roll.
    pub bonus: i32,
}

/// One set of dice with the same count of sides.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase", deny_unknown_fields))]
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

/// Modifiers on a dice roll.
///
/// # Usage in scripts:
///
/// * x -> Explode
/// * k -> Keep
/// * d -> Drop
/// * r -> Reroll
/// * c -> Count
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase", deny_unknown_fields))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiceModifier {
    /// Only considers these die and ignores all others.
    Keep(DiceSelector),
    /// Ignores the results of the given die.
    Drop(DiceSelector),
    /// Rerolls the die and takes the new result. Does not repeat
    Reroll(DiceSelector),
    /// Rerolls the die and adds the result together. May happen multiple times on the same die.
    Explode(DiceSelector),
    /// Counts the amount of dice fulfilling the selector.
    Count(DiceSelector),
}

/// Selects a group of dice in a given dice roll.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase", deny_unknown_fields))]
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

#[cfg(test)]
mod tests {
    use super::{
        CSCollection, CalculatedValue, Dice, DiceModifier, DiceSelector, DiceValue, Feature,
        FeatureModifier, FeatureSet, Limiter, PropertyDefinition, Script, Selector,
        StaticValueType,
    };

    #[cfg(feature = "serde_json")]
    const FULL_COLLECTION_JSON: &str = include_str!("../resources/tests/full_collection.json");

    #[test]
    #[cfg(feature = "serde_json")]
    fn serde() {
        let collection = get_full_collection();

        let collection_json = serde_json::to_string_pretty(&collection).unwrap();
        println!("{}", collection_json);
        assert_eq!(
            FULL_COLLECTION_JSON, collection_json,
            "The serialized JSON does match the expected one."
        );

        let collection_deserialized = serde_json::from_str(&collection_json).unwrap();
        assert_eq!(
            collection, collection_deserialized,
            "The deserialized JSON does match the original one."
        );
    }

    fn get_full_collection() -> CSCollection {
        return CSCollection {
            properties: vec![PropertyDefinition {
                name: "property1".to_string(),
                selector: Selector {
                    identifier: "selector1".to_string(),
                    arguments: vec!["arg1".to_string(), "arg2".to_string()],
                },
                limiters: vec![Limiter {
                    identifier: "limiter1".to_string(),
                    arguments: vec!["arg1".to_string()],
                }],
            }],
            items: vec![FeatureSet {
                name: "feature_set1".to_string(),
                description: "This is feature set 1.".to_string(),
                features: vec![Feature {
                    name: "feature1".to_string(),
                    description: "This is feature 1 of feature set 1.".to_string(),
                    base_type: "basic".to_string(),
                    modifiers: vec![
                        FeatureModifier {
                            property: "property1".to_string(),
                            value: CalculatedValue::StaticValue(StaticValueType::Number(1)),
                        },
                        FeatureModifier {
                            property: "property2".to_string(),
                            value: CalculatedValue::StaticValue(StaticValueType::Dice(DiceValue {
                                dice: vec![Dice {
                                    sides: 6,
                                    amount: 1,
                                    modifiers: vec![
                                        DiceModifier::Keep(DiceSelector::All),
                                        DiceModifier::Keep(DiceSelector::Highest(1)),
                                        DiceModifier::Keep(DiceSelector::Lowest(1)),
                                        DiceModifier::Reroll(DiceSelector::HigherThan(5)),
                                        DiceModifier::Explode(DiceSelector::LowerThan(2)),
                                        DiceModifier::Count(DiceSelector::Exactly(6)),
                                    ],
                                }],
                                bonus: 5,
                            })),
                        },
                        FeatureModifier {
                            property: "property3".to_string(),
                            value: CalculatedValue::Script(Script {
                                script: "5 + 6".to_string(),
                                dependencies: vec!["property1".to_string()],
                            }),
                        },
                    ],
                }],
            }],
        };
    }
}
