use std::io::{self, Write};

use crate::{
    builtins::types::Type,
    config::config::Config,
    ir::{
        heap_value::HeapValue,
        ir::IRData,
        stack_values::{PointerValue, StackValue},
    },
    vm::instructions::Instruction,
};

pub struct VMData {
    pub instructions: Vec<Instruction>,
    pub initial_heap: Vec<HeapValue>,
}

pub struct VM {
    config: Config,

    instructions: Vec<Instruction>,

    heap: Vec<HeapValue>,
}

impl VM {
    pub fn init(config: Config, vm_data: VMData) -> VM {
        VM {
            config,
            instructions: vm_data.instructions,
            heap: vm_data.initial_heap.clone(),
        }
    }
}

impl VM {
    pub fn interpret(&mut self) {
        let mut stack: Vec<StackValue> = Vec::new();
        // let mut heap: Vec<HeapValue> = default_heap.to_vec();

        let mut index = 0;

        let mut frame_index: isize = -1;

        while index < self.instructions.len() {
            let instruction = self.instructions.get(index).unwrap();
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
                        if let StackValue::I32(i) = v {
                            print!("{}", i);
                        } else if let StackValue::Char(c) = v {
                            print!("{}", c);
                        } else if let StackValue::Ptr(p) = v {
                            let heap_value = self.heap.get(p.allocation).unwrap();
                            match heap_value.r#type {
                                Type::Char => {
                                    let mut s: String = "".to_string();
                                    let mut i = p.offset;
                                    loop {
                                        if i > heap_value.len {
                                            panic!("Invalid print");
                                        }

                                        let c = heap_value.data[i];
                                        if c == 0 {
                                            break;
                                        } else {
                                            s.push(heap_value.data[i] as char);
                                        }
                                        i += 1;
                                    }

                                    print!("{}", s);
                                }
                                _ => unreachable!(),
                            }
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
                    } else if let StackValue::Char(v1) = v1
                        && let StackValue::Char(v2) = v2
                    {
                        let output = match instruction {
                            Instruction::Equal => StackValue::Bool(v1 == v2),
                            Instruction::NotEqual => StackValue::Bool(v1 != v2),
                            _ => unreachable!("Unhandled"),
                        };
                        stack.push(output);
                    } else {
                        unreachable!("Unhandled type");
                    }
                }
                //
                Instruction::Nth(n) => {
                    let rec = stack.pop().unwrap();

                    if let StackValue::Union(entries) = rec {
                        stack.push(entries.get(*n).unwrap().clone());
                    } else {
                        unreachable!("Unhandled nth type");
                    }
                }
                Instruction::NthWrite(n) => {
                    let v = stack.pop().unwrap();
                    let rec = stack.pop().unwrap();

                    if let StackValue::Union(mut entries) = rec {
                        *entries.get_mut(*n).unwrap() = v;

                        stack.push(StackValue::Union(entries));
                    } else {
                        unreachable!("Unhandled nth type");
                    }
                }
                Instruction::Union(size) => {
                    let mut values = Vec::new();
                    for _ in 0..*size {
                        values.push(stack.pop().unwrap());
                    }
                    values = values.iter().rev().cloned().collect();

                    stack.push(StackValue::Union(Box::new(values)));
                }
                //
                Instruction::Jump(offset) => {
                    index = *offset;

                    continue;
                }
                Instruction::CondFalseJump(offset_false) => {
                    let value = stack.pop().unwrap();
                    if let StackValue::Bool(b) = value {
                        if !b {
                            index = *offset_false;
                            continue;
                        }
                    } else {
                        unreachable!("Expected bool type");
                    }
                }
                Instruction::Add => {
                    let v2 = stack.pop().unwrap();
                    let v1 = stack.pop().unwrap();

                    let iv2 = match v2 {
                        StackValue::I32(v) => v,
                        _ => unreachable!(),
                    };

                    if let StackValue::I32(v1) = v1 {
                        stack.push(StackValue::I32(v1 + iv2));
                    } else if let StackValue::Ptr(p) = v1 {
                        stack.push(StackValue::Ptr(PointerValue {
                            allocation: p.allocation,
                            is_constant: p.is_constant,
                            offset: p.offset + iv2 as usize,
                        }))
                    } else {
                        unreachable!();
                    }
                }
                Instruction::Subtract
                | Instruction::Multiply
                | Instruction::Divide
                | Instruction::Modulo => {
                    let v2 = stack.pop().unwrap();
                    let v1 = stack.pop().unwrap();

                    if let StackValue::I32(v1) = v1
                        && let StackValue::I32(v2) = v2
                    {
                        let new_value = match instruction {
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
                Instruction::Call(args, call_index) => {
                    let mut arguments = Vec::new();
                    for _ in 0..*args {
                        arguments.push(stack.pop().unwrap());
                    }

                    stack.push(StackValue::RetAddr(index + 1));
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
                            StackValue::RetAddr(i) => {
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
                    let value = stack.pop().unwrap();
                    match value {
                        StackValue::VarRef(depth, slot) => {
                            let mut frame = frame_index;
                            for _ in 0..depth {
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
                        StackValue::Ptr(p) => {
                            let value = self.heap.get(p.allocation).unwrap();
                            assert!(value.len > p.offset, "Invalid Length");
                            stack.push(Self::load_value(value, p.offset));
                        }
                        _ => unreachable!(),
                    };
                }
                Instruction::Assign => {
                    let value = stack.pop().unwrap();
                    let var_pointer = stack.pop().unwrap();

                    match var_pointer {
                        StackValue::VarRef(depth, slot) => {
                            let mut frame = frame_index;
                            for _ in 0..depth {
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
                        StackValue::Ptr(ref p) => {
                            if p.is_constant {
                                unreachable!();
                            }
                            Self::store_value(
                                &value,
                                &mut self.heap.get_mut(p.allocation).unwrap(),
                                p.offset,
                            );
                        }
                        _ => unreachable!(),
                    }
                }
                Instruction::Input => {
                    let mut buffer = String::new();
                    match io::stdout().flush() {
                        Ok(_) => (),
                        Err(_) => unreachable!(),
                    };

                    match io::stdin().read_line(&mut buffer) {
                        Ok(_) => {
                            let handle = self.heap.len();

                            buffer.push('\0');

                            self.heap.push(HeapValue {
                                r#type: Type::Char,
                                len: buffer.len(),
                                data: buffer.into_boxed_str().into_boxed_bytes(),
                            });

                            stack.push(StackValue::Ptr(PointerValue {
                                allocation: handle,
                                is_constant: false,
                                offset: 0,
                            }));
                        }
                        Err(_) => unreachable!(),
                    };
                }
                Instruction::Mem => {
                    let size = if let StackValue::I32(i) = stack.pop().unwrap() {
                        i as usize
                    } else {
                        unreachable!("Invalid")
                    };

                    let r#type = if let StackValue::Type(t) = stack.pop().unwrap() {
                        t
                    } else {
                        unreachable!("Invalid");
                    };

                    let elem_size = match r#type.clone() {
                        Type::I32 => std::mem::size_of::<i32>(),
                        Type::Char => 1,
                        Type::Bool => 1,
                        _ => unreachable!(),
                    };

                    let handle = self.heap.len();

                    self.heap.push(HeapValue {
                        r#type,
                        len: size,
                        data: vec![0u8; elem_size * size].into_boxed_slice(),
                    });

                    stack.push(StackValue::Ptr(PointerValue {
                        allocation: handle,
                        is_constant: false,
                        offset: 0,
                    }));
                }
                Instruction::FrameCreate(args) => {
                    let mut values = Vec::new();
                    for _ in 0..*args {
                        values.push(stack.pop().unwrap());
                    }

                    stack.push(StackValue::Frame(frame_index));
                    frame_index = (stack.len() - 1) as isize;

                    for v in values.iter().rev() {
                        stack.push(v.clone());
                    }
                }
                Instruction::FrameRemove(args) => {
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

                    for v in values[0..(values.len() - args)].iter().rev() {
                        stack.push(v.clone());
                    }
                }
                Instruction::Lookup(d, s) => {
                    stack.push(StackValue::VarRef(*d, *s));
                }
                Instruction::DebugPrintStack => {
                    println!("{:?}", stack);
                }
                Instruction::DebugHeapStack => {
                    println!("{:?}", self.heap);
                }
            }
            index += 1;
        }
    }

    fn load_value(heap_element: &HeapValue, offset: usize) -> StackValue {
        match heap_element.r#type {
            Type::I32 => {
                let start = offset * std::mem::size_of::<i32>();

                StackValue::I32(i32::from_le_bytes(
                    heap_element.data[start..(start + 4)].try_into().unwrap(),
                ))
            }
            Type::Char => StackValue::Char(heap_element.data[offset].try_into().unwrap()),
            Type::Bool => StackValue::Bool(heap_element.data[offset].try_into().unwrap()),
            _ => unreachable!(),
        }
    }

    fn store_value(value: &StackValue, heap_element: &mut HeapValue, offset: usize) {
        match heap_element.r#type {
            Type::I32 => {
                let start = offset * std::mem::size_of::<i32>();

                let value = match value {
                    StackValue::I32(v) => v,
                    _ => unreachable!(),
                };

                heap_element.data[start..(start + 4)].copy_from_slice(&value.to_le_bytes());
            }
            Type::Char => {
                let value = match value {
                    StackValue::Char(c) => c,
                    _ => unreachable!(),
                };

                heap_element.data[offset] = *value as u8;
            }
            Type::Bool => {
                let value = match value {
                    StackValue::Bool(b) => b,
                    _ => unreachable!(),
                };

                heap_element.data[offset] = *value as u8;
            }
            _ => unreachable!("{}", heap_element.r#type),
        }
    }
}
