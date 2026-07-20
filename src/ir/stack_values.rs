use crate::builtins::types::Type;

use std::fmt;

#[derive(Debug, Clone)]
pub enum StackValue {
    I32(i32),
    Bool(bool),
    Char(char),
    Type(Type),
    Ptr(PointerValue),
    Union(Box<Vec<StackValue>>),

    Frame(isize),
    RetAddr(usize),
    VarRef(usize, usize),
}

#[derive(Debug, Clone)]
pub struct PointerValue {
    pub allocation: usize,
    pub is_constant: bool,
    pub offset: usize,
}

impl StackValue {
    pub fn from_type(t: &Type) -> StackValue {
        match t {
            Type::I32 => StackValue::I32(0),
            Type::Bool => StackValue::Bool(false),
            Type::Char => StackValue::Char('\0'),
            Type::Type(t) => StackValue::Type(*t.clone()),
            Type::Ptr(_) => panic!(),
            Type::Union(u) => StackValue::Union(Box::new(
                u.types.iter().map(|t| StackValue::from_type(t)).collect(),
            )),
            Type::RecordIden(_) => panic!(),
            Type::Var(_) => panic!(),
        }
    }
}

impl fmt::Display for StackValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StackValue::I32(i) => write!(f, "I32({i})"),
            StackValue::Bool(b) => write!(f, "BOOL({b})"),
            StackValue::Char(c) => write!(f, "CHAR({c})"),
            StackValue::Type(t) => write!(f, "TYPE({t})"),
            StackValue::Ptr(ptr) => write!(f, "PTR({ptr})"),
            StackValue::Union(values) => {
                write!(f, "UNION([")?;
                for (i, v) in (0..).zip(values.iter()) {
                    write!(f, "{v}")?;
                    if i < values.len() - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "])")
            }

            StackValue::Frame(index) => write!(f, "FRAME({index})"),
            StackValue::RetAddr(pos) => write!(f, "RET_ADDR({pos})"),
            StackValue::VarRef(depth, offset) => write!(f, "VAR_REF({depth}, {offset})"),
        }
    }
}

impl fmt::Display for PointerValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}@{}",
            if self.is_constant { "const " } else { "" },
            self.allocation,
            self.offset
        )
    }
}
