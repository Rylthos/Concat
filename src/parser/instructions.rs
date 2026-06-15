use crate::lexer::tokens::Types;

#[derive(Debug, Clone)]
pub enum StackValue {
    String(String),
    Number(f64),
    Bool(bool),
    Type(Types),
    Ptr(usize),
}

#[derive(Debug)]
pub enum Instruction {
    Push(StackValue),
    Pop,

    Duplicate,
    Drop,
    Cast,
    Print,

    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,

    Jump(usize),
    CondJump(usize, usize),

    Add,
    Subtract,
    Multiply,
    Divide,
}
