use crate::parser::stack_values::StackValue;

#[derive(Debug, Clone)]
pub enum Instruction {
    Push(StackValue),

    Rotate3,
    Duplicate,
    Drop,
    Over,
    Swap,
    Print,

    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,

    Jump(isize),
    CondJump(usize, usize),

    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    Not,

    Halt,

    Call(usize),
    Ret,

    Read,
    Assign,

    Input,

    Mem,

    FrameCreate,
    FrameRemove,

    Lookup(usize, usize),

    DebugPrintStack,
    DebugHeapStack,
}
