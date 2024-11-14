#![procedural::magic_macro]
//! Write these enums to represent a mathematical expression:
//! - One enum is called Operation and can be: Add , Sub , Mul , Div .
//! - One enum is called Expression an can be:
//!   - Number (contain inside an i32)
//!   - Operation (contain inside a left Expression, a right Expression and an Operation)
//!
//! Note: the left and right expression must be wrapped around a Box
//!   You will see Boxes further into the course, from now you just need to know that you can
//!   build a box using: Box::new(value);
//!   and you can get the value inside the box by dereferencing it: let value_inside = *my_box
//!
//! Write a function evaluate_expression that take as input an Expression, and return a
//! Result with a i32 if the result is evaluated correctly, or a string if an error occurs.

use std::fmt::Display;

#[runtest(1.0, evaluate_expression)]
///checks the general case of evaluate_expression
pub fn evaluate_expression() {
    let exp = Expression::Operation {
        left: Box::new(Expression::Operation {
            left: Box::new(Expression::Operation {
                left: Box::new(Expression::Number(4)),
                op: Operation::Mul,
                right: Box::new(Expression::Number(100)),
            }),
            op: Operation::Div,
            right: Box::new(Expression::Number(25)),
        }),
        op: Operation::Sub,
        right: Box::new(Expression::Number(100)),
    };

    assert_eq!(evaluate_expression(&exp), Result::Ok(-84));
}

#[runtest(1.0, evaluate_expression)]
///checks the case when some value is exeeded
pub fn test_overflow() {
    let exp = Expression::Operation {
        left: Box::new(Expression::Number(i32::MAX - 3)),
        op: Operation::Add,
        right: Box::new(Expression::Number(4)),
    };

    assert_eq!(evaluate_expression(&exp), Result::Err("overflow"))
}

#[runtest(1.0, evaluate_expression)]
/// checks the division by zero
pub fn test_zero_div() {
    let exp = Expression::Operation {
        left: Box::new(Expression::Number(12345)),
        op: Operation::Div,
        right: Box::new(Expression::Number(0)),
    };

    assert_eq!(evaluate_expression(&exp), Result::Err("division by zero"))
}

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

enum Expression {
    Operation {
        left: Box<Expression>,
        op: Operation,
        right: Box<Expression>,
    },
    Number(i32),
}

fn evaluate_expression(expression: &Expression) -> Result<i32, &str> {
    match expression {
        Expression::Operation { left, op, right } => {
            let val_left = evaluate_expression(left)?;
            let val_right = evaluate_expression(right)?;

            match op {
                Operation::Add => {
                    let r = val_left.checked_add(val_right);
                    match r {
                        Option::None => Result::Err("overflow"),
                        Option::Some(v) => Result::Ok(v),
                    }
                }
                Operation::Sub => {
                    let r = val_left.checked_sub(val_right);
                    match r {
                        Option::None => Result::Err("overflow"),
                        Option::Some(v) => Result::Ok(v),
                    }
                }
                Operation::Mul => {
                    let r = val_left.checked_mul(val_right);
                    match r {
                        Option::None => Result::Err("overflow"),
                        Option::Some(v) => Result::Ok(v),
                    }
                }
                Operation::Div => {
                    let r = val_left.checked_div(val_right);
                    match r {
                        Option::None => Result::Err("division by zero"),
                        Option::Some(v) => Result::Ok(v),
                    }
                }
            }
        }
        Expression::Number(n) => Result::Ok(*n),
    }
}
