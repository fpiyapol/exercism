#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Add => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b + a);
            }
            CalculatorInput::Subtract => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b - a);
            }
            CalculatorInput::Multiply => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b * a);
            }
            CalculatorInput::Divide => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b / a);
            }
            CalculatorInput::Value(val) => stack.push(val.to_owned()),
        }
    }

    if stack.len() > 1 {
        return None;
    }

    stack.pop()
}
