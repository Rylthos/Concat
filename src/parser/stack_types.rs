use crate::lexer::tokens::Types;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum StackType {
    String,
    I32,
    Bool,
    Type,
    Var(Box<StackType>),
}

impl StackType {
    pub fn convert_type(t: &Types) -> StackType {
        match t {
            Types::String => StackType::String,
            Types::I32 => StackType::I32,
            Types::Bool => StackType::Bool,
            Types::Void => unreachable!("Void should not be visible within parser"),
            Types::Type => StackType::Type,
        }
    }
}

impl fmt::Display for StackType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StackType::String => write!(f, "STRING"),
            StackType::I32 => write!(f, "I32"),
            StackType::Bool => write!(f, "BOOL"),
            StackType::Type => write!(f, "TYPE"),
            StackType::Var(v) => write!(f, "Var({})", *v),
        }
    }
}
