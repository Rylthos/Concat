use std::fmt;

use crate::parser::stack_types::StackType;

#[derive(Debug, Clone)]
pub enum Intrinsic {
    // Arithmetic Operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    // Stack Operations
    Rotate3,
    Duplicate,
    Drop,
    Over,
    Swap,
    Print,

    // Boolean Operations
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,
    Not,

    Assign,
    Read,

    Lookup(usize, usize),

    Jump(isize),
    CondJump(usize, usize),

    Identifier(String),

    Mem,

    Ret,
    Call(usize),

    StackType(StackType),

    StringValue(String),
    I32Value(i32),
    BoolValue(bool),

    FrameCreate,
    FrameRemove,

    FuncLabelDecl(String, Box<Intrinsic>),
    FuncLabelRef(String, Box<Intrinsic>),

    DebugPrintStack,

    Halt,
}

impl fmt::Display for Intrinsic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Intrinsic::Add => write!(f, "+"),
            Intrinsic::Subtract => write!(f, "-"),
            Intrinsic::Multiply => write!(f, "*"),
            Intrinsic::Divide => write!(f, "/"),
            Intrinsic::Modulo => write!(f, "%"),
            Intrinsic::Rotate3 => write!(f, "rot3"),
            Intrinsic::Duplicate => write!(f, "dup"),
            Intrinsic::Drop => write!(f, "drop"),
            Intrinsic::Over => write!(f, "over"),
            Intrinsic::Swap => write!(f, "swap"),
            Intrinsic::Print => write!(f, "print"),
            Intrinsic::Less => write!(f, "<"),
            Intrinsic::Greater => write!(f, ">"),
            Intrinsic::LessEqual => write!(f, "<="),
            Intrinsic::GreaterEqual => write!(f, ">="),
            Intrinsic::Equal => write!(f, "=="),
            Intrinsic::NotEqual => write!(f, "!="),
            Intrinsic::And => write!(f, "&&"),
            Intrinsic::Or => write!(f, "||"),
            Intrinsic::Not => write!(f, "!"),
            Intrinsic::Assign => write!(f, "="),
            Intrinsic::Read => write!(f, "@"),
            Intrinsic::Lookup(d, s) => write!(f, "lookup({d}, {s})"),
            Intrinsic::Jump(d) => write!(f, "jump({d})"),
            Intrinsic::CondJump(t, fa) => write!(f, "condJump({t}, {fa})"),
            Intrinsic::Identifier(s) => write!(f, "iden({s})"),
            Intrinsic::Mem => write!(f, "mem"),
            Intrinsic::Ret => write!(f, "ret"),
            Intrinsic::Call(i) => write!(f, "call({i})"),
            Intrinsic::StackType(t) => write!(f, "{t}"),
            Intrinsic::StringValue(s) => write!(f, "STRING {s}"),
            Intrinsic::I32Value(i) => write!(f, "I32 {i}"),
            Intrinsic::BoolValue(b) => write!(f, "BOOL {b}"),
            Intrinsic::FrameCreate => write!(f, "frameCreate"),
            Intrinsic::FrameRemove => write!(f, "frameRemove"),
            Intrinsic::FuncLabelDecl(iden, intri) => write!(f, "FuncDecl({iden}, {intri})"),
            Intrinsic::FuncLabelRef(iden, intri) => write!(f, "FuncRef({iden}, {intri})"),
            Intrinsic::DebugPrintStack => write!(f, "__print_stack__"),
            Intrinsic::Halt => write!(f, "halt"),
        }
    }
}
