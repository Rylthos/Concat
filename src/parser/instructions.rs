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

    Cast,
    Print,

    Add,
    Subtract,
    Multiply,
    Divide,
}
