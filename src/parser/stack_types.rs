use crate::lexer::tokens::TokenType;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StackType {
    String,
    I32,
    Bool,
    Char,
    Var(Box<StackType>),
    Ptr(Box<StackType>),
}

impl StackType {
    pub fn convert_type(t: &TokenType) -> StackType {
        match t {
            TokenType::String => StackType::String,
            TokenType::I32 => StackType::I32,
            TokenType::Bool => StackType::Bool,
            TokenType::Char => StackType::Char,
            _ => unreachable!("Invalid Type"),
        }
    }
}

impl fmt::Display for StackType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StackType::String => write!(f, "STRING"),
            StackType::I32 => write!(f, "I32"),
            StackType::Bool => write!(f, "BOOL"),
            StackType::Char => write!(f, "CHAR"),
            StackType::Var(v) => write!(f, "Var({})", *v),
            StackType::Ptr(p) => write!(f, "Ptr({})", *p),
        }
    }
}
