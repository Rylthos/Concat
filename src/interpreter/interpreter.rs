use crate::lexer::tokens::Types;
use crate::parser::instructions::{Instruction, StackValue};

fn cast_string(value: StackValue) -> StackValue {
    match value {
        StackValue::String(s) => StackValue::String(s),
        StackValue::Number(n) => StackValue::String(n.to_string()),
        StackValue::Bool(b) => StackValue::String(b.to_string()),
        _ => panic!("Unhandled cast to string"),
    }
}

fn cast_type(value: StackValue, target_type: Types) -> StackValue {
    match target_type {
        Types::String => cast_string(value),
        _ => panic!("Unhandled casts"),
    }
}

pub fn interpret(instructions: &Vec<Instruction>) {
    let mut stack: Vec<StackValue> = Vec::new();

    for instruction in instructions {
        match instruction {
            Instruction::Push(value) => {
                stack.push(value.clone());
            }
            Instruction::Pop => {
                stack.pop();
            }
            Instruction::Cast => {
                assert!(stack.len() >= 2, "Invalid stack length");
                let stack_type = stack.pop().unwrap();
                let stack_value = stack.pop().unwrap();

                if let StackValue::Type(t) = stack_type {
                    stack.push(cast_type(stack_value, t))
                } else {
                    panic!("Expected type");
                }
            }
            Instruction::Print => {
                let value = stack.pop();
                if let Some(v) = value {
                    if let StackValue::String(s) = v {
                        print!("{}", s);
                    } else {
                        panic!("Expected String Value");
                    }
                } else {
                    panic!("Stack Empty");
                }
            }
            Instruction::Add
            | Instruction::Subtract
            | Instruction::Multiply
            | Instruction::Divide => {
                assert!(stack.len() >= 2, "Invalid stack length");
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();

                if let StackValue::Number(v1) = v1
                    && let StackValue::Number(v2) = v2
                {
                    let new_value = match instruction {
                        Instruction::Add => StackValue::Number(v1 + v2),
                        Instruction::Subtract => StackValue::Number(v1 - v2),
                        Instruction::Multiply => StackValue::Number(v1 * v2),
                        Instruction::Divide => StackValue::Number(v1 / v2),
                        _ => panic!("Unhandled value"),
                    };
                    stack.push(new_value);
                } else {
                    panic!("Expected number types");
                }
            }
            _ => todo!("Unhandled Instruction: {:?}", instruction),
        }
    }
}
