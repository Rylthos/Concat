use crate::builtins::basic_types::BasicType;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ReducedBuiltin {
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

    Nth(usize),
    NthWrite(usize),
    Union(usize),

    Record(String),
    RecordType(String),

    BasicType(BasicType),

    StringValue(String),
    I32Value(i32),
    BoolValue(bool),
    CharValue(char),

    DebugPrintStack,
    DebugHeapStack,
}

impl fmt::Display for ReducedBuiltin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReducedBuiltin::Add => write!(f, "+"),
            ReducedBuiltin::Subtract => write!(f, "-"),
            ReducedBuiltin::Multiply => write!(f, "*"),
            ReducedBuiltin::Divide => write!(f, "/"),
            ReducedBuiltin::Modulo => write!(f, "%"),

            ReducedBuiltin::Rotate3 => write!(f, "rot3"),
            ReducedBuiltin::Duplicate => write!(f, "dup"),
            ReducedBuiltin::Drop => write!(f, "drop"),
            ReducedBuiltin::Over => write!(f, "over"),
            ReducedBuiltin::Swap => write!(f, "swap"),

            ReducedBuiltin::Print => write!(f, "print"),

            ReducedBuiltin::Less => write!(f, "<"),
            ReducedBuiltin::Greater => write!(f, ">"),
            ReducedBuiltin::LessEqual => write!(f, "<="),
            ReducedBuiltin::GreaterEqual => write!(f, ">="),
            ReducedBuiltin::Equal => write!(f, "=="),
            ReducedBuiltin::NotEqual => write!(f, "!="),
            ReducedBuiltin::And => write!(f, "&&"),
            ReducedBuiltin::Or => write!(f, "||"),

            ReducedBuiltin::Assign => write!(f, "="),
            ReducedBuiltin::Read => write!(f, "@"),

            ReducedBuiltin::Input => write!(f, "input"),

            ReducedBuiltin::Mem => write!(f, "mem"),

            ReducedBuiltin::Nth(n) => write!(f, "nth({n})"),
            ReducedBuiltin::NthWrite(n) => write!(f, "nth!({n})"),
            ReducedBuiltin::Union(s) => write!(f, "union({s})"),

            ReducedBuiltin::Record(name) => write!(f, "record({name})"),
            ReducedBuiltin::RecordType(name) => write!(f, "record_type({name})"),

            ReducedBuiltin::BasicType(t) => write!(f, "TYPE {t}"),

            ReducedBuiltin::StringValue(s) => write!(f, "STRING {:?}", s),
            ReducedBuiltin::I32Value(i) => write!(f, "I32 {i}"),
            ReducedBuiltin::BoolValue(b) => write!(f, "BOOL {b}"),
            ReducedBuiltin::CharValue(c) => write!(f, "CHAR {c}"),

            ReducedBuiltin::DebugPrintStack => write!(f, "__print_stack__"),
            ReducedBuiltin::DebugHeapStack => write!(f, "__print_heap__"),
        }
    }
}
