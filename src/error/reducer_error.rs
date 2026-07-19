use crate::{
    builtins::{basic_types::BasicType, builtins::Builtin},
    lexer::tokens::{PositionInfo, Token},
};

#[derive(Debug)]
pub enum ReducerError {
    ExpectedIntConstant(PositionInfo),
    InvalidDefine(PositionInfo),
}

impl ReducerError {
    pub fn print(&self) {
        match self {
            Self::ExpectedIntConstant(pos) => {
                eprintln!("[REDUCER] [{pos}] Expected integer constant")
            }
            Self::InvalidDefine(pos) => {
                eprintln!("[REDUCER] [{pos}] Invalid define")
            }
        }
    }
}
