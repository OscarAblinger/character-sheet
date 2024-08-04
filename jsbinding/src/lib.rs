mod utils;

use std::collections::HashMap;

use engine::CharacterSheet;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logS(s: String);
}

// global hashmap of identifiers -> CharacterSheet instances
static mut CHARSHEETS: Option<HashMap<String, CharacterSheet>> = Option::None;

fn get_charsheet(name: &str) -> Option<&mut CharacterSheet> {
    let hashmap: &mut HashMap<String, CharacterSheet>;
    unsafe {
        hashmap = CHARSHEETS.get_or_insert_with(|| HashMap::new());
    }
    return hashmap.get_mut(name);
}

fn set_charsheet(name: &str, new_charsheet: CharacterSheet) {
    let hashmap: &mut HashMap<String, CharacterSheet>;
    unsafe {
        hashmap = CHARSHEETS.get_or_insert_with(|| HashMap::new());
    }
    hashmap.insert(name.to_string(), new_charsheet);
}

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
}

fn as_string<T: serde::ser::Serialize>(value: &T) -> String {
    match serde_json::to_string(value) {
        Ok(json_string) => return json_string,
        Err(err) => return err.to_string(),
    }
}

#[wasm_bindgen(js_name = "createFromJson")]
pub fn create_from_json(name: &str, json: &str) -> JsValue {
    match serde_json::from_str(json) {
        Ok(new_charsheet) => {
            set_charsheet(name, new_charsheet);
            return JsValue::TRUE;
        }
        Err(err) => return JsValue::from_str(&("serde_json: ".to_string() + &err.to_string())),
    }
}

#[wasm_bindgen(js_name = "getAsJson")]
pub fn get_as_json(name: &str) -> String {
    match get_charsheet(name) {
        None => return "null".to_string(),
        Some(charsheet) => return as_string(charsheet),
    }
}

#[wasm_bindgen(js_name = "findMinimumRequiredUserValues")]
pub fn find_minimum_required_user_values(name: &str) -> Vec<String> {
    match get_charsheet(name) {
        None => return vec![],
        Some(charsheet) => {
            let mut uvals: Vec<String> = charsheet
                .find_minimum_required_user_values()
                .into_iter()
                .collect();
            uvals.sort();
            return uvals;
        }
    }
}

#[wasm_bindgen(js_name = "calculateAllValuesAsJson")]
pub fn calculate_all_values_as_json(name: &str) -> String {
    match get_charsheet(name) {
        None => return as_string(&Result::<&str, &str>::Ok("{}")),
        Some(charsheet) => return as_string(&charsheet.calculate_all_values()),
    }
}

#[wasm_bindgen(js_name = "setUserValueFromJson")]
pub fn set_user_value_from_json(cs_name: &str, value_name: &str, value_value_as_json: &str) -> JsValue {
    match get_charsheet(cs_name) {
        None => return JsValue::FALSE,
        Some(charsheet) => match serde_json::from_str(value_value_as_json) {
            Ok(user_values) => {
                charsheet.user_values.insert(value_name.to_string(), user_values);
                return JsValue::TRUE;
            }
            Err(err) => return JsValue::from_str(&("serde_json: ".to_string() + &err.to_string())),
        },
    }
}
