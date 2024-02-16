use cs_types::CharacterDefinition;
use cs_types::PropertyDefinition;
use std::collections::HashMap;

pub fn evaluate_character(char_def: CharacterDefinition) -> HashMap<String, PropertyDefinition> {
    return char_def.fields;
}
