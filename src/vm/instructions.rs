use crate::{builtins::types::Type, ir::stack_values::StackValue};

#[derive(Debug)]
pub enum Instruction {
    Push(StackValue),

    Add,
    Divide,
    Modulo,
    Multiply,
    Subtract,

    Drop,
    Duplicate,
    Over,
    Swap,
    Rotate3,
    Print,

    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,

    Assign,
    Read,

    Input,

    Mem,

    Nth(usize),
    NthWrite(usize),
    Union(usize),

    DebugPrintStack,
    DebugHeapStack,

    Jump(usize),
    CondFalseJump(usize),
    Call(usize, usize),
    Ret,
    FrameCreate(usize),
    FrameRemove(usize),
    Lookup(usize, usize),
    Halt,
}
