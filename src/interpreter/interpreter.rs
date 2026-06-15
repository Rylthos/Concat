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

    let mut iter = instructions.iter();
    let mut index = 0;

    while index < instructions.len() {
        let instruction = instructions.get(index).unwrap();
        match instruction {
            Instruction::Push(value) => {
                stack.push(value.clone());
            }
            Instruction::Pop => {
                stack.pop();
            }
            //
            Instruction::Rotate3 => {
                assert!(stack.len() >= 3, "Invalid stack length");
                let v3 = stack.pop().unwrap();
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();

                stack.push(v3);
                stack.push(v1);
                stack.push(v2);
            }
            Instruction::Duplicate => {
                assert!(stack.len() >= 1, "Invalid stack length");
                let stack_value = stack.last().unwrap();
                stack.push(stack_value.clone());
            }
            Instruction::Drop => {
                assert!(stack.len() >= 1, "Invalid stack length");
                stack.pop();
            }
            Instruction::Over => {
                assert!(stack.len() >= 2, "Invalid stack length");
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();
                stack.push(v1.clone());
                stack.push(v2.clone());
                stack.push(v1.clone());
            }
            Instruction::Swap => {
                assert!(stack.len() >= 2, "Invalid stack length");
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();
                stack.push(v2.clone());
                stack.push(v1.clone());
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
            //
            Instruction::Less
            | Instruction::Greater
            | Instruction::LessEqual
            | Instruction::GreaterEqual
            | Instruction::Equal
            | Instruction::NotEqual => {
                assert!(stack.len() >= 2, "Invalid stack length");
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();

                if let StackValue::Number(v1) = v1
                    && let StackValue::Number(v2) = v2
                {
                    let output = match instruction {
                        Instruction::Less => StackValue::Bool(v1 < v2),
                        Instruction::Greater => StackValue::Bool(v1 > v2),
                        Instruction::LessEqual => StackValue::Bool(v1 <= v2),
                        Instruction::GreaterEqual => StackValue::Bool(v1 >= v2),
                        Instruction::Equal => StackValue::Bool(v1 == v2),
                        Instruction::NotEqual => StackValue::Bool(v1 != v2),
                        _ => panic!("Unhandled"),
                    };
                    stack.push(output);
                } else {
                    panic!("Unhandled type");
                }
            }
            //
            Instruction::Jump(offset) => {
                index += offset;
            }
            Instruction::CondJump(offset_true, offset_false) => {
                assert!(stack.len() >= 1, "Invalid stack length");
                let value = stack.pop().unwrap();
                if let StackValue::Bool(b) = value {
                    let offset = match b {
                        true => offset_true,
                        false => offset_false,
                    };
                    index += offset;
                } else {
                    panic!("Expected bool type");
                }
            }
            Instruction::BackJump(value) => {
                index -= value + 1;
            }
            //
            Instruction::Add
            | Instruction::Subtract
            | Instruction::Multiply
            | Instruction::Divide
            | Instruction::Modulo => {
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
                        Instruction::Modulo => StackValue::Number(v1 % v2),
                        _ => panic!("Unhandled value"),
                    };
                    stack.push(new_value);
                } else {
                    panic!("Expected number types");
                }
            }
        }
        index += 1;
    }
}
