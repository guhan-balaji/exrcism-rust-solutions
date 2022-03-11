use core::panic;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::with_capacity(inputs.len());
    let mut iterator = inputs.into_iter();
    while let Some(input) = iterator.next() {
        match input {
            CalculatorInput::Value(value) => stack.push(*value),
            operator => {
                if stack.len() < 2 {
                    return None;
                } else {
                    arithmetic_op_on_2_vars_and_push(&mut stack, operator);
                }
            }
        }
    }
    (stack.len() == 1).then(|| stack.pop().unwrap())
}

fn arithmetic_op_on_2_vars_and_push(stack: &mut Vec<i32>, operator: &CalculatorInput) {
    let rhs = stack.pop().unwrap();
    let lhs = stack.pop().unwrap();

    use self::CalculatorInput::*;
    match operator {
        Add => stack.push(lhs + rhs),
        Subtract => stack.push(lhs - rhs),
        Multiply => stack.push(lhs * rhs),
        Divide => stack.push(lhs / rhs),
        _ => panic!("'{:?}' is not an arithmetic operator", operator),
    }
}
