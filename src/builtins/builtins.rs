use crate::builtins::basic_types::BasicType;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Builtin {
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

    Syscall,

    RawNth,
    RawNthWrite,
    RawUnion,

    Define,

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

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Builtin::Add => write!(f, "+"),
            Builtin::Subtract => write!(f, "-"),
            Builtin::Multiply => write!(f, "*"),
            Builtin::Divide => write!(f, "/"),
            Builtin::Modulo => write!(f, "%"),

            Builtin::Rotate3 => write!(f, "rot3"),
            Builtin::Duplicate => write!(f, "dup"),
            Builtin::Drop => write!(f, "drop"),
            Builtin::Over => write!(f, "over"),
            Builtin::Swap => write!(f, "swap"),

            Builtin::Print => write!(f, "print"),

            Builtin::Less => write!(f, "<"),
            Builtin::Greater => write!(f, ">"),
            Builtin::LessEqual => write!(f, "<="),
            Builtin::GreaterEqual => write!(f, ">="),
            Builtin::Equal => write!(f, "=="),
            Builtin::NotEqual => write!(f, "!="),
            Builtin::And => write!(f, "&&"),
            Builtin::Or => write!(f, "||"),

            Builtin::Assign => write!(f, "="),
            Builtin::Read => write!(f, "@"),

            Builtin::Input => write!(f, "input"),

            Builtin::Mem => write!(f, "mem"),

            Builtin::Syscall => write!(f, "syscall"),

            Builtin::RawNth => write!(f, "nth"),
            Builtin::RawNthWrite => write!(f, "nth!"),
            Builtin::RawUnion => write!(f, "union"),

            Builtin::Define => write!(f, "define"),

            Builtin::Record(name) => write!(f, "record({name})"),
            Builtin::RecordType(name) => write!(f, "record_type({name})"),

            Builtin::BasicType(t) => write!(f, "TYPE {t}"),

            Builtin::StringValue(s) => write!(f, "STRING {:?}", s),
            Builtin::I32Value(i) => write!(f, "I32 {i}"),
            Builtin::BoolValue(b) => write!(f, "BOOL {b}"),
            Builtin::CharValue(c) => write!(f, "CHAR {c}"),

            Builtin::DebugPrintStack => write!(f, "__print_stack__"),
            Builtin::DebugHeapStack => write!(f, "__print_heap__"),
        }
    }
}
