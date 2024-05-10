use crate::character_sheet::{Selector, StaticValueType};

#[derive(Debug)]
pub struct FirstSelector {}

impl Selector for FirstSelector {
    fn select(&self, mut items: Vec<StaticValueType>) -> StaticValueType {
        items.remove(0)
    }

    fn string_representation(&self) -> String {
        String::from("FirstSelector")
    }
}
