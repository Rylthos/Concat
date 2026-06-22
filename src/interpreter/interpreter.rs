use crate::parser::instructions::{Instruction, StackValue};

pub fn interpret(instructions: &Vec<Instruction>) {
    let mut stack: Vec<StackValue> = Vec::new();

    let mut index = 0;

    let mut frame_index: isize = -1;

    while index < instructions.len() {
        let instruction = instructions.get(index).unwrap();
        match instruction {
            Instruction::Push(value) => {
                stack.push(value.clone());
            }
            //
            Instruction::Rotate3 => {
                let v3 = stack.pop().unwrap();
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();

                stack.push(v3);
                stack.push(v1);
                stack.push(v2);
            }
            Instruction::Duplicate => {
                let stack_value = stack.last().unwrap();
                stack.push(stack_value.clone());
            }
            Instruction::Drop => {
                stack.pop();
            }
            Instruction::Over => {
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();
                stack.push(v1.clone());
                stack.push(v2.clone());
                stack.push(v1.clone());
            }
            Instruction::Swap => {
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
                        unreachable!("Expected printable Value");
                    }
                } else {
                    unreachable!("Stack Empty");
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
                        _ => unreachable!("Unhandled"),
                    };
                    stack.push(output);
                } else if let StackValue::Bool(b1) = v1
                    && let StackValue::Bool(b2) = v2
                {
                    let output = match instruction {
                        Instruction::And => StackValue::Bool(b1 && b2),
                        Instruction::Or => StackValue::Bool(b1 || b2),
                        _ => unreachable!("Unhandled"),
                    };
                    stack.push(output);
                } else {
                    unreachable!("Unhandled type");
                }
            }
            Instruction::Not => {
                let v1 = stack.pop().unwrap();
                if let StackValue::Bool(b) = v1 {
                    stack.push(StackValue::Bool(!b));
                } else {
                    unreachable!("Unhandled type");
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
                let value = stack.pop().unwrap();
                if let StackValue::Bool(b) = value {
                    let offset = match b {
                        true => offset_true,
                        false => offset_false,
                    };
                    index += offset;
                    continue;
                } else {
                    unreachable!("Expected bool type");
                }
            }
            Instruction::Add
            | Instruction::Subtract
            | Instruction::Multiply
            | Instruction::Divide
            | Instruction::Modulo => {
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
                        _ => unreachable!("Unhandled value"),
                    };
                    stack.push(new_value);
                } else {
                    unreachable!(
                        "Expected number types: {:?} got {:?} {:?}",
                        instruction, v1, v2
                    );
                }
            }
            Instruction::Halt => {
                break;
            }
            Instruction::Call(call_index) => {
                let num_arguments = if let StackValue::I32(i) = stack.pop().unwrap() {
                    i
                } else {
                    unreachable!("Invalid call")
                };

                let mut arguments = Vec::new();
                for _ in 0..num_arguments {
                    arguments.push(stack.pop().unwrap());
                }

                stack.push(StackValue::Call(index + 1));
                index = *call_index;

                for v in arguments.iter().rev() {
                    stack.push(v.clone());
                }
                continue;
            }
            Instruction::Ret => {
                let mut arguments = Vec::new();
                while let Some(s) = stack.pop() {
                    match s {
                        StackValue::Call(i) => {
                            index = i;
                            break;
                        }
                        _ => {
                            arguments.push(s);
                        }
                    }
                }

                for v in arguments.iter().rev() {
                    stack.push(v.clone());
                }
                continue;
            }
            Instruction::Read => {
                let var_pointer = stack.pop().unwrap();
                let (depth, slot) = match var_pointer {
                    StackValue::VarRef(d, s) => (d, s),
                    _ => unreachable!(),
                };
                let mut frame = frame_index;
                for i in 0..depth {
                    if let Some(s) = stack.get(frame as usize) {
                        match s {
                            StackValue::Frame(i) => frame = *i,
                            _ => unreachable!(),
                        }
                    } else {
                        unreachable!()
                    }
                }

                stack.push(stack.get((frame as usize) + slot + 1).unwrap().clone());
            }
            Instruction::Assign => {
                let value = stack.pop().unwrap();

                let var_pointer = stack.pop().unwrap();
                let (depth, slot) = match var_pointer {
                    StackValue::VarRef(d, s) => (d, s),
                    _ => unreachable!(),
                };

                let mut frame = frame_index;
                for i in 0..depth {
                    if let Some(s) = stack.get(frame as usize) {
                        match s {
                            StackValue::Frame(i) => frame = *i,
                            _ => unreachable!(),
                        }
                    } else {
                        unreachable!()
                    }
                }

                if let Some(s) = stack.get_mut((frame as usize) + slot + 1) {
                    *s = value;
                } else {
                    unreachable!();
                };
            }
            Instruction::FrameCreate => {
                let num_values = if let StackValue::I32(i) = stack.pop().unwrap() {
                    i
                } else {
                    unreachable!("Invalid call")
                };

                let mut values = Vec::new();
                for _ in 0..num_values {
                    values.push(stack.pop().unwrap());
                }

                stack.push(StackValue::Frame(frame_index));
                frame_index = (stack.len() - 1) as isize;

                for v in values.iter().rev() {
                    stack.push(v.clone());
                }
            }
            Instruction::FrameRemove => {
                let num_values: usize = if let StackValue::I32(i) = stack.pop().unwrap() {
                    i as usize
                } else {
                    unreachable!("Invalid call")
                };

                let mut values = Vec::new();
                while let Some(s) = stack.pop() {
                    match s {
                        StackValue::Frame(i) => {
                            frame_index = i;
                            break;
                        }
                        _ => {
                            values.push(s);
                        }
                    }
                }

                for v in values[0..(values.len() - num_values)].iter().rev() {
                    stack.push(v.clone());
                }
            }
            Instruction::Lookup(d, s) => {
                stack.push(StackValue::VarRef(*d, *s));
            }
            Instruction::FuncLabelDecl(_, _) | Instruction::FuncLabelRef(_, _) => {
                panic!("Pseudo instructions: Should not be executed");
            }
        }
        index += 1;
    }
}
