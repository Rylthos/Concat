use crate::builtins::basic_types::BasicType;

use std::fmt;

#[derive(Debug, Clone)]
pub enum Type {
    I32,
    Bool,
    Char,
    Type(Box<Type>),
    Ptr(Box<PtrType>),
    Union(Box<UnionType>),
    RecordIden(String),
    Var(Box<Type>),
}

#[derive(Debug, Clone)]
pub struct PtrType {
    pub is_const: bool,
    pub r#type: Type,
}

#[derive(Debug, Clone)]
pub struct UnionType {
    pub types: Vec<Type>,
}

impl Type {
    pub fn from_basic_type(basic_type: &BasicType) -> Type {
        match basic_type {
            BasicType::I32 => Type::I32,
            BasicType::Bool => Type::Bool,
            BasicType::Char => Type::Char,
            BasicType::Ptr(p) => Type::Ptr(Box::new(PtrType {
                is_const: p.is_const,
                r#type: Type::from_basic_type(&p.r#type),
            })),
            BasicType::Union(u) => Type::Union(Box::new(UnionType {
                types: u.types.iter().map(|t| Type::from_basic_type(&t)).collect(),
            })),
            BasicType::RecordIden(s) => Type::RecordIden(s.clone()),
            _ => panic!(),
        }
    }
    pub fn can_become(&self, target: &Type) -> bool {
        match (self, target) {
            (Type::I32, Type::I32) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::Char, Type::Char) => true,
            (Type::Type(t1), Type::Type(t2)) => t1.can_become(t2),
            (Type::Union(t1), Type::Union(t2)) => {
                t1.types.len() >= t2.types.len()
                    && t1
                        .types
                        .iter()
                        .zip(t2.types.iter())
                        .map(|(a, b)| a.can_become(b))
                        .all(|t| t)
            }
            (Type::RecordIden(s1), Type::RecordIden(s2)) => s1 == s2,
            (Type::Ptr(p1), Type::Ptr(p2)) => {
                p1.r#type.can_become(&p2.r#type) && (p2.is_const || (!p2.is_const && !p1.is_const))
            }
            (Type::Var(v1), Type::Var(v2)) => v1.can_become(v2),
            _ => false,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::I32 => write!(f, "I32"),
            Type::Bool => write!(f, "BOOL"),
            Type::Char => write!(f, "CHAR"),
            Type::Type(t) => write!(f, "TYPE({t})"),
            Type::Ptr(ptr) => write!(f, "PTR({ptr})"),
            Type::Union(union) => write!(f, "UNION({union})"),
            Type::RecordIden(name) => write!(f, "RECORD({name})"),
            Type::Var(t) => write!(f, "VAR({t})"),
        }
    }
}

impl fmt::Display for PtrType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if self.is_const { "const " } else { "" },
            self.r#type
        )
    }
}

impl fmt::Display for UnionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, t) in (0..).zip(self.types.iter()) {
            write!(f, "{}", t)?;
            if i < self.types.len() - 1 {
                write!(f, " ")?;
            }
        }
        write!(f, "]")
    }
}
