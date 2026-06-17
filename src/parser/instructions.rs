use crate::lexer::tokens::Types;

#[derive(Debug, Clone)]
pub enum StackValue {
    String(String),
    I32(i32),
    Bool(bool),
    Type(Types),
    Ptr(usize),
}

#[derive(Debug)]
pub enum Instruction {
    Push(StackValue),
    Pop,

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
}
