use types::character_sheet::{Dice, DiceValue, StaticValueType};

fn num(value: i32) -> StaticValueType {
    return StaticValueType::Number(value);
}

#[test]
fn arithmetic_functions_number_number() {
    assert_eq!(num(3) + num(2), num(5));
    assert_eq!(num(0) + num(1), num(1));
    assert_eq!(num(-15) + num(5), num(-10));

    assert_eq!(num(15) - num(5), num(10));
    assert_eq!(num(0) - num(1), num(-1));

    assert_eq!(num(10) * num(5), num(50));
    assert_eq!(num(0) * num(1), num(0));
    assert_eq!(num(10) * num(-2), num(-20));

    assert_eq!(num(15) / num(5), Some(num(3)));
    assert_eq!(num(19) / num(5), Some(num(3)));
    assert_eq!(num(0) / num(2), Some(num(0)));
    assert_eq!(num(10) / num(-2), Some(num(-5)));
    assert_eq!(num(-10) / num(2), Some(num(-5)));
    assert_eq!(num(-10) / num(0), None);
}

fn dice1(amount: i32, sides: u32, bonus: i32) -> StaticValueType {
    return StaticValueType::Dice(DiceValue {
        dice: vec![Dice {
            amount,
            sides,
            modifiers: vec![],
        }],
        bonus,
    });
}

fn dice2(amount1: i32, sides1: u32, amount2: i32, sides2: u32, bonus: i32) -> StaticValueType {
    return StaticValueType::Dice(DiceValue {
        dice: vec![
            Dice {
                amount: amount1,
                sides: sides1,
                modifiers: vec![],
            },
            Dice {
                amount: amount2,
                sides: sides2,
                modifiers: vec![],
            },
        ],
        bonus,
    });
}

fn dice3(amount1: i32, sides1: u32, amount2: i32, sides2: u32, amount3: i32, sides3: u32, bonus: i32) -> StaticValueType {
    return StaticValueType::Dice(DiceValue {
        dice: vec![
            Dice {
                amount: amount1,
                sides: sides1,
                modifiers: vec![],
            },
            Dice {
                amount: amount2,
                sides: sides2,
                modifiers: vec![],
            },
            Dice {
                amount: amount3,
                sides: sides3,
                modifiers: vec![],
            },
        ],
        bonus,
    });
}

#[test]
fn arithmetic_functions_number_dice() {
    // +
    assert_eq!(num(3) + dice1(2, 6, 0), dice1(2, 6, 3));
    assert_eq!(num(0) + dice1(1, 6, -2), dice1(1, 6, -2));
    assert_eq!(num(-2) + dice1(1, 6, 3), dice1(1, 6, 1));

    assert_eq!(dice1(2, 6, 0) + num(3), dice1(2, 6, 3));
    assert_eq!(dice1(1, 6, -2) + num(0), dice1(1, 6, -2));
    assert_eq!(dice1(1, 6, 3) + num(-2), dice1(1, 6, 1));

    // -
    assert_eq!(num(3) - dice1(1, 6, 0), dice1(1, 6, 3));
    assert_eq!(num(0) - dice1(2, 6, -2), dice1(2, 6, 2));
    assert_eq!(num(-2) - dice1(3, 6, 3), dice1(3, 6, -5));

    assert_eq!(dice1(1, 6, 0) - num(3), dice1(1, 6, -3));
    assert_eq!(dice1(2, 6, -2) - num(0), dice1(2, 6, -2));
    assert_eq!(dice1(3, 6, 3) - num(-2), dice1(3, 6, 5));

    // *
    assert_eq!(num(3) * dice1(1, 6, 0),  dice1(3, 6, 0));
    assert_eq!(num(0) * dice1(2, 6, -2), num(0));
    assert_eq!(num(-2) * dice1(3, 6, 3), dice1(-6, 6, -6));
    assert_eq!(num(2) * dice2(1, 4, -3, 6, 2),  dice2(2, 4, -6, 6, 4));

    assert_eq!(dice1(1, 6, 0) * num(3),  dice1(3, 6, 0));
    assert_eq!(dice1(2, 6, -2) * num(0), num(0));
    assert_eq!(dice1(3, 6, 3) * num(-2), dice1(-6, 6, -6));
    assert_eq!(dice2(1, 4, -3, 6, 2) * num(2),  dice2(2, 4, -6, 6, 4));

    // /
    assert_eq!(num(3) / dice1(1, 6, 0),  None);
    assert_eq!(num(0) / dice1(2, 6, -4), Some(dice1(2, 6, 0)));
    assert_eq!(num(4) / dice1(2, 6, -3), Some(dice1(2, 6, -1)));
    assert_eq!(num(-2) / dice1(4, 6, 3), Some(dice1(4, 6, 0)));
    assert_eq!(num(4) / dice2(1, 4, -3, 6, 2),  Some(dice2(1, 4, -3, 6, 2)));

    assert_eq!(dice1(1, 6, 0) / num(3),  Some(dice1(1, 6, 0)));
    assert_eq!(dice1(1, 6, 5) / num(3),  Some(dice1(1, 6, 1)));
    assert_eq!(dice1(1, 6, 5) / num(-1),  Some(dice1(1, 6, -5)));
    assert_eq!(dice1(1, 6, -5) / num(-1),  Some(dice1(1, 6, 5)));
    assert_eq!(dice1(1, 6, 5) / num(0),  None);
    assert_eq!(dice2(1, 4, -3, 6, 4) / num(2),  Some(dice2(1, 4, -3, 6, 2)));
}

#[test]
fn arithmetic_functions_dice_dice() {
    // +
    assert_eq!(dice1(1, 6, 2) + dice1(2, 6, 3), dice1(3, 6, 5));
    assert_eq!(dice1(1, 4, 2) + dice1(2, 6, 3), dice2(1, 4, 2, 6, 5));
    assert_eq!(dice2(1, 4, 2, 6, 2) + dice2(-1, 6, -1, 8, 3), dice3(1, 4, 1, 6, -1, 8, 5));

    // -
    assert_eq!(dice1(1, 6, 2) - dice1(2, 6, 3), dice1(-1, 6, -1));
    assert_eq!(dice1(2, 6, 2) - dice1(2, 6, 3), num(-1));
    assert_eq!(dice2(1, 4, 2, 6, 2) - dice2(2, 6, 1, 8, 3), dice2(1, 4, -1, 8, -1));
    assert_eq!(dice1(1, 4, 2) - dice1(2, 6, 3), dice2(1, 4, -2, 6, -1));
    assert_eq!(dice2(1, 4, 2, 6, 3) - dice2(-1, 6, -1, 8, 3), dice3(1, 4, 3, 6, 1, 8, 0));

    // *
    assert_eq!(dice1(2, 6, 2) * dice1(2, 6, 3), dice1(4, 6, 6));
    assert_eq!(dice1(2, 4, 2) * dice1(2, 6, 3), num(6));

    // /
    assert_eq!(dice1(2, 6, 3) / dice1(2, 6, 2), Some(dice1(1, 6, 1)));
    assert_eq!(dice1(2, 4, 3) / dice1(2, 6, 2), None);
}
