use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::{Add, Div, Mul, Sub};

#[cfg(feature = "serde")]
use serde::de::Visitor;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// As the name implies a feature set bundles a bunch of features together.
/// In most games this may be anything from classes to races to items or even spells in some cases.
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct FeatureSet {
    pub name: String,
    pub description: String,
    pub source: String,
    pub features: Vec<Feature>,
}

/// A feature is any actual value that a character may have.
/// This can range from things like HP or Mana all the way to Attacks and spells.
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Feature {
    pub name: String,
    pub description: String,
    /// The base types should mostly be specified by the base rules for the game system.
    /// They help you categorize it into the proper sections of your UI as well as use them in
    /// filters etc..
    pub base_type: String,
    /// A definition specifies how a property may look like.
    /// It can also be used to have a feature specify a property, that you expect the user to
    /// provide the value of. E.g. basic attributes of your character. Or your characters level.
    pub definitions: Vec<PropertyDefinition>,
    /// The modifiers specify which values will be changed how, if this feature is active.
    pub modifiers: Vec<FeatureModifier>,
}

/// Definition of the type of a property
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
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
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Selector {
    /// The identifier of the selector.
    /// Example: `highest`
    /// The list of supported selectors depends on the execution engine.
    pub identifier: String,
    /// Potential arguments for the selector.
    pub arguments: Vec<String>,
}

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Limiter {
    /// The identifier of the limiter.
    /// Example: `maximum`
    /// The list of supported limiters depends on the execution engine.
    pub identifier: String,
    /// Potential arguments for the limiter.
    pub arguments: Vec<String>,
}

/// A set of changes to a given property.
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct FeatureModifier {
    /// The property of the character that this references.
    pub property: String,
    /// The changes applied to the property.
    pub value: CalculatedValue,
}

/// A set of operations that will result in a specific value.
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CalculatedValue {
    /// A static value always has the same value.
    StaticValue(StaticValueType),
    /// A script is a value that depends on some operations and usually other properties.
    CalculationValue(Calculation),
}

impl Default for CalculatedValue {
    fn default() -> Self {
        Self::StaticValue(StaticValueType::default())
    }
}

/// An actual value that a property may have.
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StaticValueType {
    Number(i32),
    Dice(DiceValue),
}

impl Default for StaticValueType {
    fn default() -> Self {
        Self::Number(0)
    }
}

fn dice_with_func<F>(template: &Dice, amount1: i32, amount2: i32, merge_function: F) -> Dice
where
    F: Fn(i32, i32) -> i32,
{
    return Dice {
        amount: merge_function(amount1, amount2),
        sides: template.sides,
        modifiers: template.modifiers.clone(),
    };
}

fn merge_dice<F>(first: Vec<Dice>, second: Vec<Dice>, merge_function: F) -> Vec<Dice>
where
    F: Fn(i32, i32) -> i32,
{
    let mut sorted_first = first.clone();
    sorted_first.sort();
    let mut sorted_second = second.clone();
    sorted_second.sort();

    let mut idx_first = 0;
    let mut idx_second = 0;

    let mut result = vec![];
    while idx_first < first.len() || idx_second < second.len() {
        match (first.get(idx_first), second.get(idx_second)) {
            (Some(a), Some(b)) if a.sides == b.sides && a.modifiers == b.modifiers => {
                idx_first += 1;
                idx_second += 1;
                result.push(dice_with_func(a, a.amount, b.amount, &merge_function));
            },
            (Some(a), Some(b)) if a < b => {
                idx_first += 1;
                result.push(dice_with_func(a, a.amount, 0, &merge_function));
            },
            // at this point we know that a >= b
            (Some(_), Some(b)) => {
                idx_second += 1;
                result.push(dice_with_func(b, 0, b.amount, &merge_function));
            },
            (Some(a), None) => {
                idx_first += 1;
                result.push(dice_with_func(a, a.amount, 0, &merge_function));
            },
            (None, Some(b)) => {
                idx_second += 1;
                result.push(dice_with_func(b, 0, b.amount, &merge_function));
            },
            (None, None) => break, // should never happen, but if it does, breaking the loop is
                                   // correct
        }
    }
    result.retain(|a| a.amount != 0);
    return result;
}

impl Add for StaticValueType {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (StaticValueType::Number(a), StaticValueType::Number(b)) => {
                return StaticValueType::Number(a + b)
            }
            (StaticValueType::Number(n), StaticValueType::Dice(d))
            | (StaticValueType::Dice(d), StaticValueType::Number(n)) => {
                return StaticValueType::Dice(DiceValue {
                    dice: d.dice,
                    bonus: d.bonus + n,
                });
            }
            (StaticValueType::Dice(a), StaticValueType::Dice(b)) => {
                let dice = merge_dice(a.dice, b.dice, |a, b| a + b);
                if dice.len() == 0 {
                    return StaticValueType::Number(a.bonus + b.bonus);
                } else {
                    return StaticValueType::Dice(DiceValue {
                        dice,
                        bonus: a.bonus + b.bonus,
                    });
                }
            }
        }
    }
}

impl Sub for StaticValueType {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (StaticValueType::Number(a), StaticValueType::Number(b)) => {
                return StaticValueType::Number(a - b)
            }
            (StaticValueType::Number(n), StaticValueType::Dice(d)) => {
                return StaticValueType::Dice(DiceValue {
                    dice: d.dice,
                    bonus: n - d.bonus,
                });
            }
            (StaticValueType::Dice(d), StaticValueType::Number(n)) => {
                return StaticValueType::Dice(DiceValue {
                    dice: d.dice,
                    bonus: d.bonus - n,
                });
            }
            (StaticValueType::Dice(a), StaticValueType::Dice(b)) => {
                let dice = merge_dice(a.dice, b.dice, |a, b| a - b);
                if dice.len() == 0 {
                    return StaticValueType::Number(a.bonus - b.bonus);
                } else {
                    return StaticValueType::Dice(DiceValue {
                        dice,
                        bonus: a.bonus - b.bonus,
                    });
                }
            }
        }
    }
}

impl Mul for StaticValueType {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (StaticValueType::Number(a), StaticValueType::Number(b)) => {
                return StaticValueType::Number(a * b)
            }
            (StaticValueType::Number(n), StaticValueType::Dice(d))
            | (StaticValueType::Dice(d), StaticValueType::Number(n)) => {
                let dice: Vec<Dice> = d
                    .dice
                    .iter()
                    .cloned()
                    .map(|mut d| {
                        d.amount *= n;
                        d
                    })
                    .filter(|d| d.amount != 0)
                    .collect();
                if dice.len() == 0 {
                    return StaticValueType::Number(d.bonus * n);
                } else {
                    return StaticValueType::Dice(DiceValue {
                        dice,
                        bonus: d.bonus * n,
                    });
                }
            }
            (StaticValueType::Dice(a), StaticValueType::Dice(b)) => {
                let dice = merge_dice(a.dice, b.dice, |a, b| a * b);
                if dice.len() == 0 {
                    return StaticValueType::Number(a.bonus * b.bonus);
                } else {
                    return StaticValueType::Dice(DiceValue {
                        dice,
                        bonus: a.bonus * b.bonus,
                    });
                }
            }
        }
    }
}

impl Div for StaticValueType {
    type Output = Option<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (StaticValueType::Number(a), StaticValueType::Number(b)) => {
                if b == 0 {
                    return None;
                } else {
                    return Some(StaticValueType::Number(a / b));
                }
            }
            (StaticValueType::Number(n), StaticValueType::Dice(d)) => {
                if d.bonus == 0 {
                    return None;
                } else {
                    return Some(StaticValueType::Dice(DiceValue {
                        dice: d.dice,
                        bonus: n / d.bonus,
                    }));
                }
            }
            (StaticValueType::Dice(d), StaticValueType::Number(n)) => {
                if n == 0 {
                    return None;
                } else {
                    return Some(StaticValueType::Dice(DiceValue {
                        dice: d.dice,
                        bonus: d.bonus / n,
                    }));
                }
            }
            (StaticValueType::Dice(a), StaticValueType::Dice(b)) => {
                let side_a: HashSet<u32> = a.dice.iter().map(|d| d.sides).collect();
                if b.bonus == 0 || b.dice.iter().any(|d| d.amount == 0 || !side_a.contains(&d.sides)) {
                    return None;
                } else {
                    let dice = merge_dice(a.dice, b.dice, |a, b| a / b);
                    if dice.len() == 0 {
                        return Some(StaticValueType::Number(a.bonus / b.bonus));
                    } else {
                        return Some(StaticValueType::Dice(DiceValue {
                            dice,
                            bonus: a.bonus / b.bonus,
                        }));
                    }
                }
            }
        }
    }
}

/// A script is a value that depends on some operations and usually other properties.
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Calculation {
    /// The list of properties that this script depends on.
    pub dependencies: Vec<String>,
    /// A stack-based, postfix description of the calculation that should be done.
    pub operations: Vec<CalculationOperation>,
}

/// Individual calculation operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CalculationOperation {
    /// An invalid value.
    /// Most operations will return another invalid value if provided this argument.
    Invalid(String) = 0,

    /// A static number.
    Integer(i32) = 10,
    /// Loads the value of the nth dependency.
    Dependency(u32) = 15,

    /// An 1-arg op that already has the number of sides and takes the number of dice as argument.
    Dice(u32) = 20,
    // An 2-arg op that takes the number of dice as 1st argument, and the sides as 2nd argument.
    Dice2 = 21,

    // 2-arg op
    Add = 30,
    // 2-arg op
    Substract = 31,
    // 2-arg op
    Multiply = 32,
    // 2-arg op.
    Divide = 33,

    // An x-arg op that calls the function specified by the string.
    // The first argument is the number of other arguments. Then arguments of that amount are
    // loaded with rest rest filled with Invalid values.
    // So e.g. vec![Integer(2), Integer(4), Integer(2), FuncX("pow")] assuming an implementation of
    // pow similar to (a, b) => a^b would evaluate to Integer(16).
    FuncX(String) = 50,
}

#[cfg(feature = "serde")]
impl Serialize for CalculationOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CalculationOperation::Invalid(v) => {
                return serializer.serialize_str(&format!("err: {}", v))
            }
            CalculationOperation::Integer(v) => return serializer.serialize_i32(*v),
            CalculationOperation::Dependency(v) => {
                return serializer.serialize_str(&format!("dep_{}", v))
            }
            CalculationOperation::Dice(v) => {
                return serializer.serialize_str(&format!("dice_{}", v))
            }
            CalculationOperation::Dice2 => return serializer.serialize_str("dice2"),
            CalculationOperation::Add => return serializer.serialize_str("+"),
            CalculationOperation::Substract => return serializer.serialize_str("-"),
            CalculationOperation::Multiply => return serializer.serialize_str("*"),
            CalculationOperation::Divide => return serializer.serialize_str("/"),
            CalculationOperation::FuncX(v) => {
                return serializer.serialize_str(&format!("func_{}", v))
            }
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for CalculationOperation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CalculationOperationVisitor)
    }
}

#[cfg(feature = "serde")]
struct CalculationOperationVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for CalculationOperationVisitor {
    type Value = CalculationOperation;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        return formatter.write_str("an CalculationOperation");
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        return Ok(CalculationOperation::Integer(v));
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        return i32::try_from(v)
            .map_err(|_| {
                E::invalid_type(
                    serde::de::Unexpected::Signed(v),
                    &"an signed number that fits in 32 bits",
                )
            })
            .and_then(|i32v| self.visit_i32(i32v));
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        return i32::try_from(v)
            .map_err(|_| {
                E::invalid_type(
                    serde::de::Unexpected::Unsigned(v),
                    &"an signed number that fits in 32 bits",
                )
            })
            .and_then(|i32v| self.visit_i32(i32v));
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            s if s.starts_with("err: ") => {
                return Ok(CalculationOperation::Invalid(
                    s["err: ".len()..].to_string(),
                ))
            }
            s if s.starts_with("dep_") => {
                return rest_as_u32(v, "dep_".len()).map(|v| CalculationOperation::Dependency(v))
            }
            s if s.starts_with("dice_") => {
                return rest_as_u32(v, "dice_".len()).map(|v| CalculationOperation::Dice(v))
            }
            "dice2" => return Ok(CalculationOperation::Dice2),
            "+" => return Ok(CalculationOperation::Add),
            "-" => return Ok(CalculationOperation::Substract),
            "*" => return Ok(CalculationOperation::Multiply),
            "/" => return Ok(CalculationOperation::Divide),
            s if s.starts_with("func_") => {
                return Ok(CalculationOperation::FuncX(s["func_".len()..].to_string()))
            }
            s => return Err(E::unknown_variant(s, &["any CalculationOperation"])),
        }
    }
}

#[cfg(feature = "serde")]
fn rest_as_u32<E>(str: &str, offset: usize) -> Result<u32, E>
where
    E: serde::de::Error,
{
    return str[offset..]
        .parse::<u32>()
        .map_err(|_| E::custom(format!("argument of '{}' is not a valid i32", str)));
}

/// A value that consists of a bunch of dice that should be rolled to get the actual value.
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DiceValue {
    /// The dice sets to be rolled.
    pub dice: Vec<Dice>,
    /// A stareeauieasult of the roll.
    pub bonus: i32,
}

/// One set of dice with the same count of sides.
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Dice {
    /// The amount of dice with this amount of sides.
    /// A negative number means that the total should be substracted.
    /// It should never be 0. Instead just remove these dice.
    pub amount: i32,
    /// The sides that each individual die has.
    pub sides: u32,
    /// Modifiers that should be applied to the dice when rolled.
    pub modifiers: Vec<DiceModifier>,
}

impl Ord for Dice {
    fn cmp(&self, other: &Self) -> Ordering {
        return self
            .sides
            .cmp(&other.sides)
            .then_with(|| self.amount.cmp(&other.amount))
            .then_with(|| self.modifiers.cmp(&other.modifiers));
    }
}

impl PartialOrd for Dice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let sides = self.sides.partial_cmp(&other.sides);
        let amount = self.amount.partial_cmp(&other.amount);
        let modifiers = self.modifiers.partial_cmp(&other.modifiers);

        match (sides, amount, modifiers) {
            (Some(sides_ord), Some(amount_ord), Some(modifiers_ord)) => {
                return Some(
                    sides_ord
                        .then_with(|| amount_ord)
                        .then_with(|| modifiers_ord),
                )
            }
            (_, _, _) => return None,
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
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
        CalculatedValue, Calculation, CalculationOperation, Dice, DiceModifier, DiceSelector,
        DiceValue, Feature, FeatureModifier, FeatureSet, Limiter, PropertyDefinition, Selector,
        StaticValueType,
    };

    #[cfg(feature = "serde_json")]
    const FULL_COLLECTION_JSON: &str = include_str!("../resources/tests/full_collection.json");

    #[test]
    #[cfg(feature = "serde_json")]
    fn serde() {
        let feature_collection = get_example_features();

        let collection_json = serde_json::to_string_pretty(&feature_collection).unwrap();

        assert_eq!(
            FULL_COLLECTION_JSON.replace("\r", "").trim(),
            collection_json.replace("\r", "").trim(),
            "The serialized JSON does match the expected one."
        );

        let collection_deserialized: Vec<FeatureSet> =
            serde_json::from_str(&collection_json).unwrap();
        assert_eq!(
            feature_collection, collection_deserialized,
            "The deserialized JSON does match the original one."
        );
    }

    fn get_example_features() -> Vec<FeatureSet> {
        return vec![FeatureSet {
            name: "feature_set1".to_string(),
            description: "This is feature set 1.".to_string(),
            source: "Basic rules".to_string(),
            features: vec![Feature {
                name: "feature1".to_string(),
                description: "This is feature 1 of feature set 1.".to_string(),
                base_type: "basic".to_string(),
                definitions: vec![PropertyDefinition {
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
                        value: CalculatedValue::CalculationValue(Calculation {
                            dependencies: vec!["property1".to_string(), "property2".to_string()],
                            operations: vec![
                                CalculationOperation::Dependency(0),
                                CalculationOperation::Integer(2),
                                CalculationOperation::Add,
                                CalculationOperation::Dependency(1),
                                CalculationOperation::Integer(1),
                                CalculationOperation::FuncX("max".to_string()),
                            ],
                        }),
                    },
                ],
            }],
        }];
    }
}
