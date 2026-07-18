use std::fmt;

use crate::lexer::tokens::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum BasicType {
    I32,
    Bool,
    Char,
    Void,
    Ptr(Box<PtrType>),
    Union(Box<UnionType>),
    RecordIden(String),
}

#[derive(Debug, Clone)]
pub struct PtrType {
    pub is_const: bool,
    pub r#type: BasicType,
}

#[derive(Debug, Clone)]
pub struct UnionType {
    pub types: Vec<BasicType>,
}

impl BasicType {
    pub fn from_token(token: &Token) -> BasicType {
        match token.token_type {
            TokenType::I32 => BasicType::I32,
            TokenType::Bool => BasicType::Bool,
            TokenType::Char => BasicType::Char,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for BasicType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BasicType::I32 => write!(f, "I32"),
            BasicType::Bool => write!(f, "BOOL"),
            BasicType::Char => write!(f, "CHAR"),
            BasicType::Void => write!(f, "VOID"),
            BasicType::Ptr(p) => write!(f, "{p}"),
            BasicType::Union(u) => write!(f, "{u}"),
            BasicType::RecordIden(i) => write!(f, "RECORD{i}"),
        }
    }
}

impl fmt::Display for PtrType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PTR {} {}",
            if self.is_const { "const" } else { "" },
            self.r#type
        )
    }
}

impl fmt::Display for UnionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UNION [")?;
        for (i, t) in (0..).zip(self.types.iter()) {
            write!(f, "{t}")?;
            if i < self.types.len() - 1 {
                write!(f, " ")?;
            }
        }
        write!(f, "]")
    }
}
