use crate::builtins::types::Type;
use std::fmt;

#[derive(Debug, Clone)]
pub enum TypedBuiltin {
    Add,
    Divide,
    Modulo,
    Multiply,
    Subtract,

    Drop,
    Duplicate,
    Over,
    Swap,
    Rotate3,
    Print,

    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,

    Assign,
    Read,

    Input,

    Mem,

    Record(String),

    Nth(usize),
    NthWrite(usize),
    Union(usize),

    Type(Type),

    StringValue(String),
    I32Value(i32),
    BoolValue(bool),
    CharValue(char),

    DebugPrintStack,
    DebugHeapStack,
}

impl fmt::Display for TypedBuiltin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypedBuiltin::Add => write!(f, "+"),
            TypedBuiltin::Subtract => write!(f, "-"),
            TypedBuiltin::Multiply => write!(f, "*"),
            TypedBuiltin::Divide => write!(f, "/"),
            TypedBuiltin::Modulo => write!(f, "%"),

            TypedBuiltin::Rotate3 => write!(f, "rot3"),
            TypedBuiltin::Duplicate => write!(f, "dup"),
            TypedBuiltin::Drop => write!(f, "drop"),
            TypedBuiltin::Over => write!(f, "over"),
            TypedBuiltin::Swap => write!(f, "swap"),

            TypedBuiltin::Print => write!(f, "print"),

            TypedBuiltin::Less => write!(f, "<"),
            TypedBuiltin::Greater => write!(f, ">"),
            TypedBuiltin::LessEqual => write!(f, "<="),
            TypedBuiltin::GreaterEqual => write!(f, ">="),
            TypedBuiltin::Equal => write!(f, "=="),
            TypedBuiltin::NotEqual => write!(f, "!="),
            TypedBuiltin::And => write!(f, "&&"),
            TypedBuiltin::Or => write!(f, "||"),

            TypedBuiltin::Assign => write!(f, "="),
            TypedBuiltin::Read => write!(f, "@"),

            TypedBuiltin::Input => write!(f, "input"),

            TypedBuiltin::Mem => write!(f, "mem"),

            TypedBuiltin::Record(name) => write!(f, "record({name})"),

            TypedBuiltin::Nth(n) => write!(f, "nth({n})"),
            TypedBuiltin::NthWrite(n) => write!(f, "nth!({n})"),
            TypedBuiltin::Union(s) => write!(f, "union({s})"),

            TypedBuiltin::Type(t) => write!(f, "TYPE {:?}", t),

            TypedBuiltin::StringValue(s) => write!(f, "STRING {:?}", s),
            TypedBuiltin::I32Value(i) => write!(f, "I32 {i}"),
            TypedBuiltin::BoolValue(b) => write!(f, "BOOL {b}"),
            TypedBuiltin::CharValue(c) => write!(f, "CHAR {c}"),

            TypedBuiltin::DebugPrintStack => write!(f, "__print_stack__"),
            TypedBuiltin::DebugHeapStack => write!(f, "__print_heap__"),
        }
    }
}
