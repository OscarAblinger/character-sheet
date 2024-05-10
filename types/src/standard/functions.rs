use crate::character_sheet::{CalculatedValue, CalculatorError, FunctionArgs, StaticValueType};

use crate::character_sheet::StaticValueType::Number;
use crate::character_sheet::StaticValueType::Dice;

pub fn add(args: FunctionArgs) -> Result<StaticValueType, CalculatorError> {
    Ok(args.iter().fold(Number(0), |acc, arg| {
        match (acc, arg) {
            (Number(x), Number(y)) => Number(x + y),
            (Dice(x), y) => Dice(x.add(&y)),
            (x, Dice(y)) => Dice(y.add(&x)),
        }
    }))
}

pub fn substract(args: Vec<CalculatedValue>) -> Result<StaticValueType, CalculatorError> {
    Err(CalculatorError::new_public("unimplemented function 'add'".to_string()))
}

pub fn divide(args: Vec<CalculatedValue>) -> Result<StaticValueType, CalculatorError> {
    Err(CalculatorError::new_public("unimplemented function 'add'".to_string()))
}

pub fn multiply(args: Vec<CalculatedValue>) -> Result<StaticValueType, CalculatorError> {
    Err(CalculatorError::new_public("unimplemented function 'add'".to_string()))
}

