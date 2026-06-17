use crate::lexer::tokens::Types;
use crate::parser::instructions::{Instruction, StackValue};

pub fn interpret(instructions: &Vec<Instruction>) {
    let mut stack: Vec<StackValue> = Vec::new();

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
            Instruction::Print => {
                let value = stack.pop();
                if let Some(v) = value {
                    if let StackValue::String(s) = v {
                        print!("{}", s);
                    } else if let StackValue::I32(i) = v {
                        print!("{}", i);
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
            | Instruction::NotEqual
            | Instruction::And
            | Instruction::Or => {
                assert!(stack.len() >= 2, "Invalid stack length");
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();

                if let StackValue::I32(v1) = v1
                    && let StackValue::I32(v2) = v2
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
                } else if let StackValue::Bool(b1) = v1
                    && let StackValue::Bool(b2) = v2
                {
                    let output = match instruction {
                        Instruction::And => StackValue::Bool(b1 && b2),
                        Instruction::Or => StackValue::Bool(b1 || b2),
                        _ => panic!("Unhandled"),
                    };
                    stack.push(output);
                } else {
                    panic!("Unhandled type");
                }
            }
            Instruction::Not => {
                assert!(stack.len() >= 1, "Invalid stack length");
                let v1 = stack.pop().unwrap();
                if let StackValue::Bool(b) = v1 {
                    stack.push(StackValue::Bool(!b));
                } else {
                    panic!("Unhandled type");
                }
            }
            //
            Instruction::Jump(offset) => {
                if *offset > 0isize {
                    index += *offset as usize;
                } else {
                    index -= -(*offset) as usize;
                }

                continue;
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
                    continue;
                } else {
                    panic!("Expected bool type");
                }
            }
            Instruction::Add
            | Instruction::Subtract
            | Instruction::Multiply
            | Instruction::Divide
            | Instruction::Modulo => {
                assert!(stack.len() >= 2, "Invalid stack length");
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();

                if let StackValue::I32(v1) = v1
                    && let StackValue::I32(v2) = v2
                {
                    let new_value = match instruction {
                        Instruction::Add => StackValue::I32(v1 + v2),
                        Instruction::Subtract => StackValue::I32(v1 - v2),
                        Instruction::Multiply => StackValue::I32(v1 * v2),
                        Instruction::Divide => StackValue::I32(v1 / v2),
                        Instruction::Modulo => StackValue::I32(v1 % v2),
                        _ => panic!("Unhandled value"),
                    };
                    stack.push(new_value);
                } else {
                    panic!(
                        "Expected number types: {:?} got {:?} {:?}",
                        instruction, v1, v2
                    );
                }
            }
            Instruction::Halt => {
                break;
            }
            Instruction::Call(index) => {
                todo!();
            }
            Instruction::Ret => {
                todo!();
            }
            Instruction::Label(_, _) | Instruction::LabelRef(_, _) => {
                panic!("Pseudo instructions: Should not be executed");
            }
        }
        index += 1;
    }
}
