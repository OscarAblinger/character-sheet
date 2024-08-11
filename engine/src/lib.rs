#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use types::character_sheet::{
    CalculatedValue, Calculation, CalculationOperation, Dice, DiceValue, FeatureModifier,
    FeatureSet, StaticValueType,
};

pub type ResultValue = Result<StaticValueType, ValueCalculationError>;

/// Value set by the user of the sheet.
/// Aka the base values like the character's class, level, characteristics etc.
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserValue {
    pub named: String,
    pub value: StaticValueType,
}

/// Errors during the calculation of a property
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueCalculationError {
    /// A cyclic dependency was found.
    Cycle(Vec<CycleNode>),
    /// Evaluation of the script threw some error.
    ScriptError(String),
    /// A property had no value or feature reference, but is required as a dependency.
    MissingDependency(MissingDependency),
}

impl std::fmt::Display for ValueCalculationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self ;
                return Ok(());
            }
        }
    }
}

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CycleNode {
    feature_set: String,
    feature: String,
    property: String,
}

/// The current values of the character sheet do not allow a calculation of any of its values.
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IllegalSheetError {}

/// The data behind a character sheet.
/// The base class of the engine.
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterSheet {
    /// User values are values explicitely set by the user of the sheet.
    /// Generally speaking this includes two different use cases:
    /// - Selections during the character creation (e.g. stat spread, level...).
    /// - Overwrites by the user.
    /// User values will always overwrite those set by features.
    pub user_values: HashMap<String, StaticValueType>,
    /// Active features will apply their modifications to the properties of the character.
    pub active_features: Vec<FeatureSet>,
    /// Inactive features will not affect the character's properties.
    /// They are still included, because oftentimes you may still want to display them.
    /// An example for this would be an item that the character is carrying, but that's not
    /// equipped.
    pub inactive_features: Vec<FeatureSet>,
}

impl CharacterSheet {
    pub fn new() -> CharacterSheet {
        return CharacterSheet {
            user_values: HashMap::new(),
            active_features: vec![],
            inactive_features: vec![],
        };
    }

    /// Finds all values that are used as dependency, but that don't have a feature that defines
    /// them.
    pub fn find_minimum_required_user_values(&self) -> HashSet<String> {
        let mut specified_properties: HashSet<String> = HashSet::new();
        let mut required_properties: HashSet<String> = HashSet::new();

        for featureset in &self.active_features {
            for feature in &featureset.features {
                for definition in &feature.definitions {
                    required_properties.insert(definition.name.clone());
                }

                for modifier in &feature.modifiers {
                    specified_properties.insert(modifier.property.clone());
                    match &modifier.value {
                        CalculatedValue::StaticValue(_) => {} // no dependencies
                        CalculatedValue::CalculationValue(calculation) => {
                            for dep in &calculation.dependencies {
                                required_properties.insert(dep.clone());
                            }
                        }
                    }
                }
            }
        }

        // All properties that were specified as a dependency, but not as a feature.
        // todo: Does not account for cycles.
        return &required_properties - &specified_properties;
    }

    /// Calculates and returns all values.
    pub fn calculate_all_values<'a>(
        &'a self,
    ) -> Result<HashMap<String, ResultValue>, IllegalSheetError> {
        let mut values: HashMap<String, ResultValue> = HashMap::new();

        self.add_user_values(&mut values);

        let mut calc_map: HashMap<String, CalcInfo<'a>> = HashMap::new();
        for feature_set in &self.active_features {
            for feature in &feature_set.features {
                for modifier in &feature.modifiers {
                    calc_map.insert(
                        modifier.property.clone(),
                        CalcInfo {
                            feature_set: &feature_set.name,
                            feature: &feature.name,
                            modifier: &modifier,
                        },
                    );
                }
            }
        }

        let mut currently_calculating: HashSet<String> = HashSet::new();
        for (_, calc_info) in &calc_map {
            self.calculate_values(
                &mut values,
                &mut currently_calculating,
                &calc_map,
                calc_info,
            );
        }

        return Ok(values);
    }

    fn add_user_values(
        &self,
        values: &mut HashMap<String, Result<StaticValueType, ValueCalculationError>>,
    ) {
        for (name, value) in &self.user_values {
            values.insert(name.clone(), Ok(value.clone()));
        }
    }

    fn calculate_values<'a>(
        &self,
        values: &mut HashMap<String, ResultValue>,
        currently_calculating: &mut HashSet<String>,
        calc_map: &'a HashMap<String, CalcInfo<'a>>,
        calc_info: &'a CalcInfo<'a>,
    ) -> AddOrCalcResult {
        let curr_property = &calc_info.modifier.property;

        if currently_calculating.contains(curr_property) {
            // the cycle node vec will be completed when resolving the recursive stack frame
            let value: ResultValue = Err(ValueCalculationError::Cycle(Vec::from([CycleNode {
                feature_set: calc_info.feature_set.to_string(),
                feature: calc_info.feature.to_string(),
                property: calc_info.modifier.property.clone(),
            }])));
            values.insert(curr_property.clone(), value);
            return AddOrCalcResult::Cycle;
        }

        match &calc_info.modifier.value {
            CalculatedValue::StaticValue(ref value) => {
                values.insert(curr_property.clone(), Ok(value.clone()));
                return AddOrCalcResult::Success;
            }
            CalculatedValue::CalculationValue(calculation) => {
                currently_calculating.insert(curr_property.clone());
                for dep in &calculation.dependencies {
                    if values.contains_key(dep) {
                        continue;
                    }

                    if !calc_map.contains_key(dep) {
                        let missing_dep = MissingDependency {
                            missing_dependency: dep.clone(),
                            found_in_feature_set: calc_info.feature_set.to_string(),
                            found_in_feature: calc_info.feature.to_string(),
                            found_in_property: calc_info.modifier.property.to_string(),
                        };

                        values.insert(
                            curr_property.clone(),
                            Err(ValueCalculationError::MissingDependency(
                                missing_dep.clone(),
                            )),
                        );
                        return AddOrCalcResult::MissingDependency(missing_dep);
                    }

                    match self.calculate_values(
                        values,
                        currently_calculating,
                        calc_map,
                        calc_map.get(dep).expect(&format!(
                            "No calc info for dep {:?}. calc map: {:?}",
                            dep, calc_map
                        )),
                    ) {
                        AddOrCalcResult::Success => {}
                        AddOrCalcResult::MissingDependency(missing_dep) => {
                            values.insert(
                                curr_property.clone(),
                                Err(ValueCalculationError::MissingDependency(
                                    missing_dep.clone(),
                                )),
                            );
                            return AddOrCalcResult::MissingDependency(missing_dep);
                        }
                        AddOrCalcResult::Cycle => {
                            // expand cycle vector and also set the value to it
                            match values.get_mut(dep) {
                                Some(Err(ValueCalculationError::Cycle(cycle))) => {
                                    cycle.insert(
                                        0,
                                        CycleNode {
                                            feature_set: calc_info.feature_set.to_string(),
                                            feature: calc_info.feature.to_string(),
                                            property: calc_info.modifier.property.clone(),
                                        },
                                    );
                                }
                                None => {
                                    panic!("Value not calculated after call to calculate_values with result Cycle.
                                           Property resolution that claims cycle: {:?}
                                           Currently calculating: {:?}
                                           Calculation map: {:?}
                                           Values until now: {:?}",
                                           dep, currently_calculating, calc_map, values);
                                }
                                Some(Ok(_)) => {
                                    panic!(
                                        "Claimed cycle does not exist.
                                           Property resolution that claims cycle: {:?}
                                           Currently calculating: {:?}
                                           Calculation map: {:?}
                                           Values until now: {:?}",
                                        dep, currently_calculating, calc_map, values
                                    );
                                }
                                Some(Err(err)) => {
                                    let err2 = err.clone();
                                    panic!(
                                        "Claimed cycle, but different error encountered.
                                                   Actual error: {:?}
                                                   Property resolution that claims cycle: {:?}
                                                   Currently calculating: {:?}
                                                   Calculation map: {:?}
                                                   Values until now: {:?}",
                                        err2, dep, currently_calculating, calc_map, values
                                    );
                                }
                            }
                            return AddOrCalcResult::Cycle;
                        }
                    }
                }
                currently_calculating.remove(curr_property);

                values.insert(
                    curr_property.clone(),
                    self.evaluate_calculation(calculation, values),
                );
                return AddOrCalcResult::Success;
            }
        }
    }

    fn evaluate_calculation(
        &self,
        calc: &Calculation,
        known_values: &HashMap<String, ResultValue>,
    ) -> ResultValue {
        let deps: Vec<&ResultValue> = calc.dependencies.iter()
            .map(|dep| {
                return known_values.get(dep)
                    .expect(&format!("Missing value for dependency during calculation (should've been caught earlier): {:?}", dep));
            })
            .collect();

        let mut stack: Vec<ResultValue> = vec![];
        for (idx, op) in calc.operations.iter().enumerate() {
            match op {
                CalculationOperation::Invalid(err) => {
                    stack.push(Err(ValueCalculationError::ScriptError(err.clone())))
                }
                CalculationOperation::Integer(val) => stack.push(Ok(StaticValueType::Number(*val))),
                CalculationOperation::Dependency(idx) => {
                    stack.push(deps.get(*idx as usize)
                        .map(|r| (*r).clone())
                        .unwrap_or_else(|| Err(ValueCalculationError::ScriptError(format!("Requested dependency {} out of bounds (dependencies are {:?}).", idx, calc.dependencies)))));
                }
                CalculationOperation::Dice(d) => {
                    let result = try_to_number(try_pop(&mut stack, idx))
                        .map(|amount| 
                            StaticValueType::Dice(DiceValue {
                                dice: vec![Dice {
                                    amount: amount,
                                    sides: *d,
                                    modifiers: vec![],
                                }],
                                bonus: 0,
                            })
                        );
                    stack.push(result);
                },
                CalculationOperation::Dice2 => {
                    match try_to_number2(try_pop2(&mut stack, idx)) {
                        (Err(e), _) | (_, Err(e)) =>
                            stack.push(Err(e)),
                        (Ok(sides), Ok(amount)) => {
                            stack.push(Ok(StaticValueType::Dice(DiceValue {
                                dice: vec![Dice {
                                    amount: amount,
                                    sides: sides as u32,
                                    modifiers: vec![],
                                }],
                                bonus: 0,
                            })));
                        }
                    }
                },
                CalculationOperation::Add => {
                    match try_pop2(&mut stack, idx) {
                        (Err(e), _) | (_, Err(e)) =>
                            stack.push(Err(e)),
                        (Ok(first), Ok(second)) => {
                            stack.push(Ok(first + second));
                        }
                    }
                },
                CalculationOperation::Substract => {
                    match try_pop2(&mut stack, idx) {
                        (Err(e), _) | (_, Err(e)) =>
                            stack.push(Err(e)),
                        (Ok(first), Ok(second)) => {
                            stack.push(Ok(first - second));
                        }
                    }
                },
                CalculationOperation::Multiply => {
                    match try_pop2(&mut stack, idx) {
                        (Err(e), _) | (_, Err(e)) =>
                            stack.push(Err(e)),
                        (Ok(first), Ok(second)) => {
                            stack.push(Ok(first * second));
                        }
                    }
                },
                CalculationOperation::Divide => {
                    match try_pop2(&mut stack, idx) {
                        (Err(e), _) | (_, Err(e)) =>
                            stack.push(Err(e)),
                        (Ok(first), Ok(second)) => {
                            match first / second {
                                Some(value) => stack.push(Ok(value)),
                                None => stack.push(Err(ValueCalculationError::ScriptError("Division by zero".to_string()))),
                            }
                        }
                    }
                },
                CalculationOperation::FuncX(name) => {
                    match try_to_number(try_pop(&mut stack, idx)) {
                        Err(e) => stack.push(Err(e)),
                        Ok(argc) => {
                            let mut args = vec![];
                            for _ in 1..argc {
                                match try_pop(&mut stack, idx) {
                                    Err(e) => {
                                        stack.push(Err(e));
                                        break;
                                    },
                                    Ok(value) => args.push(value),
                                }
                            }
                            // check if we actually got all args.
                            // otherwise we already reported and error and have nothing left to do.
                            if args.len() == argc as usize {
                                args.reverse();

                                stack.push(call_function(name, args));
                            }
                        },
                    }
                },
            }
        }

        if stack.len() > 1 {
            return Err(ValueCalculationError::ScriptError(format!("Stack overfull: {} elements left at the end of program execution.", stack.len())));
        } else if stack.len() == 0 {
            return Err(ValueCalculationError::ScriptError("Stack underfull: No elements left at the end of program execution.".to_string()));
        } else {
            return stack.remove(0);
        }
    }
}

fn call_function(name: &str, args: Vec<StaticValueType>) -> Result<StaticValueType, ValueCalculationError> {
    match name {
        "max" => args.iter().max().map(|v| v.clone()).ok_or(ValueCalculationError::ScriptError("No arguments for max()".to_string())),
        _ => Err(ValueCalculationError::ScriptError(format!("Unknown function {}", name))),
    }
}

fn try_pop(stack: &mut Vec<ResultValue>, idx: usize) -> ResultValue {
    return stack.pop()
        .unwrap_or_else(|| Err(ValueCalculationError::ScriptError(format!("StackUnderflow: trying to pop an empty stack at operation with index {}.", idx))));
}

fn try_to_number(result_val: ResultValue) -> Result<i32, ValueCalculationError> {
    return result_val
        .and_then(|value| {
            match value {
                StaticValueType::Number(number) => Ok(number),
                StaticValueType::Dice(_) => Err(ValueCalculationError::ScriptError("Expected a number on the stack, but found dice instead.".to_string())),
            }
        });
}

fn try_pop2(stack: &mut Vec<ResultValue>, idx: usize) -> (ResultValue, ResultValue) {
    let first = stack.pop()
        .unwrap_or_else(|| Err(ValueCalculationError::ScriptError(format!("StackUnderflow: trying to pop an empty stack at operation with index {}.", idx))));
    let second = stack.pop()
        .unwrap_or_else(|| Err(ValueCalculationError::ScriptError(format!("StackUnderflow: trying to pop an empty stack at operation with index {}.", idx))));
    return (first, second);
}

fn try_to_number2(result_vals: (ResultValue, ResultValue)) -> (Result<i32, ValueCalculationError>, Result<i32, ValueCalculationError>) {
    let first = result_vals.0
        .and_then(|value| {
            match value {
                StaticValueType::Number(number) => Ok(number),
                StaticValueType::Dice(_) => Err(ValueCalculationError::ScriptError("Expected a number on the stack, but found dice instead.".to_string())),
            }
        });
    let second = result_vals.1
        .and_then(|value| {
            match value {
                StaticValueType::Number(number) => Ok(number),
                StaticValueType::Dice(_) => Err(ValueCalculationError::ScriptError("Expected a number on the stack, but found dice instead.".to_string())),
            }
        });
    return (first, second);
}

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum AddOrCalcResult {
    Success,
    Cycle,
    MissingDependency(MissingDependency),
}

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingDependency {
    missing_dependency: String,
    found_in_feature_set: String,
    found_in_feature: String,
    found_in_property: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CalcInfo<'a> {
    pub feature_set: &'a str,
    pub feature: &'a str,
    pub modifier: &'a FeatureModifier,
}

fn dump<T: std::fmt::Debug>(str: &str, val: T) -> T {
    println!("{}{:?}", str, val);
    return val;
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use types::character_sheet::{
        CalculatedValue, Calculation, CalculationOperation, Feature, FeatureModifier, FeatureSet,
        StaticValueType,
    };

    use crate::ResultValue;

    #[test]
    fn complex_test() {
        //let mut collection = CSCollection::new();
        let mut sheet = super::CharacterSheet::new();

        sheet.active_features.push(FeatureSet {
            name: "base".to_string(),
            description: "The base rules".to_string(),
            source: "BasicRules".to_string(),
            features: vec![Feature {
                name: "Attributes".to_string(),
                description: "Your character has basic attributes.".to_string(),
                base_type: "base_rules".to_string(),
                definitions: vec![],
                modifiers: vec![FeatureModifier {
                    property: "MeleeAttack".to_string(),
                    value: CalculatedValue::CalculationValue(Calculation {
                        dependencies: vec!["Strength".to_string()],
                        operations: vec![
                            CalculationOperation::Dependency(0),
                            CalculationOperation::Integer(1),
                            CalculationOperation::Add,
                        ],
                    }),
                }],
            }],
        });

        let mut expected_values: HashMap<String, ResultValue> = HashMap::new();
        expected_values.insert(
            "MeleeAttack".to_string(),
            Err(crate::ValueCalculationError::MissingDependency(
                crate::MissingDependency {
                    missing_dependency: "Strength".to_string(),
                    found_in_feature_set: "base".to_string(),
                    found_in_feature: "Attributes".to_string(),
                    found_in_property: "MeleeAttack".to_string(),
                },
            )),
        );
        assert_eq!(
            sheet.calculate_all_values(),
            Ok(expected_values),
            "Missing dependency"
        );

        let mut expected_required_user_values = HashSet::new();
        expected_required_user_values.insert("Strength".to_string());
        assert_eq!(
            sheet.find_minimum_required_user_values(),
            expected_required_user_values,
            "The missing dependency is required as user value."
        );

        sheet
            .user_values
            .insert("Strength".to_string(), StaticValueType::Number(10));

        assert_eq!(
            sheet.find_minimum_required_user_values(),
            expected_required_user_values,
            "The expected user values stay the same, even if one is provided."
        );

        let mut expected_values: HashMap<String, ResultValue> = HashMap::new();
        expected_values.insert("MeleeAttack".to_string(), Ok(StaticValueType::Number(11)));
        expected_values.insert("Strength".to_string(), Ok(StaticValueType::Number(10)));
        assert_eq!(
            sheet.calculate_all_values(),
            Ok(expected_values),
            "Once Strength is provided as user value, it can be evaluated."
        );
    }
}
