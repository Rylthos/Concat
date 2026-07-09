use crate::lexer::tokens::TokenType;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StackType {
    I32,
    Bool,
    Char,
    Var(Box<StackType>),
    Ptr(bool, Box<StackType>),
    Union(Vec<Box<StackType>>),
    RecordIden(String),
}

impl StackType {
    pub fn convert_type(t: &TokenType) -> StackType {
        match t {
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
            StackType::I32 => write!(f, "I32"),
            StackType::Bool => write!(f, "BOOL"),
            StackType::Char => write!(f, "CHAR"),
            StackType::Var(v) => write!(f, "Var({})", *v),
            StackType::Ptr(c, p) => write!(f, "Ptr({}, {})", *c, *p),
            StackType::Union(records) => {
                write!(f, "Union([")?;
                for i in 0..records.len() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", records[i])?
                }
                write!(f, "])")
            }
            StackType::RecordIden(name) => {
                write!(f, "RecordIden({:?})", name)
            }
        }
    }
}
