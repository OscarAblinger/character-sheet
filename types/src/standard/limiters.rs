use crate::character_sheet::{Limiter, StaticValueType};

#[derive(Debug)]
pub struct NoLimiter {}

impl Limiter for NoLimiter {
    fn apply_limits(&self, _value: &mut StaticValueType) {
        // do nothing
    }

    fn string_representation(&self) -> String {
        String::from("NoLimiter")
    }
}


