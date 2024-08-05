#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use types::character_sheet_collection::{
    CalculatedValue, FeatureModifier, FeatureSet, Script, StaticValueType,
};

pub type ResultValue = Result<StaticValueType, ValueCalculationError>;

/// Value set by the user of the sheet.
/// Aka the base values like the character's class, level, characteristics etc.
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
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
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueCalculationError {
    /// A cyclic dependency was found.
    Cycle(Vec<CycleNode>),
    /// Evaluation of the script threw some error.
    ScriptError(String),
    /// A property had no value or feature reference, but is required as a dependency.
    MissingDependency(MissingDependency),
}

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
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
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IllegalSheetError {}

/// The data behind a character sheet.
/// The base class of the engine.
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
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
                        CalculatedValue::Script(script) => {
                            for dep in &script.dependencies {
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
            self.add_or_calc(
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

    fn add_or_calc<'a>(
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
            CalculatedValue::Script(script) => {
                currently_calculating.insert(curr_property.clone());
                for dep in &script.dependencies {
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

                    match self.add_or_calc(
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
                                    panic!("Value not calculated after call to add_or_calc with result Cycle.
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

                values.insert(curr_property.clone(), self.evaluate_script(script, values));
                return AddOrCalcResult::Success;
            }
        }
    }

    fn evaluate_script(
        &self,
        script: &Script,
        _values: &HashMap<String, ResultValue>,
    ) -> ResultValue {
        // todo: proper parsing
        // for now we only parse integers
        return match script.script.parse::<i32>() {
            Ok(val) => Ok(StaticValueType::Number(val)),
            Err(err) => Err(ValueCalculationError::ScriptError(err.to_string())),
        };
    }
}

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
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
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingDependency {
    missing_dependency: String,
    found_in_feature_set: String,
    found_in_feature: String,
    found_in_property: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
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

    use types::character_sheet_collection::{
        CSCollection, CalculatedValue, Feature, FeatureModifier, FeatureSet, Script,
        StaticValueType,
    };

    use crate::ResultValue;

    #[test]
    fn complex_test() {
        let mut collection = CSCollection::new();
        let mut sheet = super::CharacterSheet::new();

        add_active_featureset(
            &mut collection,
            &mut sheet,
            FeatureSet {
                name: "base".to_string(),
                description: "The base rules".to_string(),
                features: vec![Feature {
                    name: "Attributes".to_string(),
                    description: "Your character has basic attributes.".to_string(),
                    base_type: "base_rules".to_string(),
                    modifiers: vec![FeatureModifier {
                        property: "MeleeAttack".to_string(),
                        value: CalculatedValue::Script(Script {
                            script: "11".to_string(), // todo: make it 1 + strength
                            dependencies: vec!["Strength".to_string()],
                        }),
                    }],
                }],
            },
        );

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

    fn add_active_featureset(
        collection: &mut CSCollection,
        sheet: &mut crate::CharacterSheet,
        fs: FeatureSet,
    ) {
        collection.items.push(fs.clone());
        sheet.active_features.push(fs);
    }
}
