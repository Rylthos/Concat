use std::fmt;

use crate::lexer::tokens::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum BasicType {
    I32,
    Bool,
    Char,
    Void,
    Ptr(Box<BasicPtrType>),
    Union(Box<BasicUnionType>),
    RecordIden(String),
}

#[derive(Debug, Clone)]
pub struct BasicPtrType {
    pub is_const: bool,
    pub r#type: BasicType,
}

#[derive(Debug, Clone)]
pub struct BasicUnionType {
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

    pub fn can_become(&self, target: &BasicType) -> bool {
        match (self, target) {
            (BasicType::I32, BasicType::I32) => true,
            (BasicType::Bool, BasicType::Bool) => true,
            (BasicType::Char, BasicType::Char) => true,
            (BasicType::Union(t1), BasicType::Union(t2)) => {
                t1.types.len() >= t2.types.len()
                    && t1
                        .types
                        .iter()
                        .zip(t2.types.iter())
                        .map(|(a, b)| a.can_become(b))
                        .all(|t| t)
            }
            (BasicType::RecordIden(s1), BasicType::RecordIden(s2)) => s1 == s2,
            (BasicType::Ptr(p1), BasicType::Ptr(p2)) => {
                p1.r#type.can_become(&p2.r#type) && (p2.is_const || (!p2.is_const && !p1.is_const))
            }
            _ => false,
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

impl fmt::Display for BasicPtrType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PTR {} {}",
            if self.is_const { "const" } else { "" },
            self.r#type
        )
    }
}

impl fmt::Display for BasicUnionType {
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
