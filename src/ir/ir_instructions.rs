use crate::ir::stack_values::StackValue;

use std::fmt;

#[derive(Clone)]
pub enum IRInstruction {
    Push(StackValue),

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

    Syscall(usize),

    Nth(usize),
    NthWrite(usize),
    Union(usize),

    DebugPrintStack,
    DebugHeapStack,

    Label(Label),
    Jump(Label),
    CondFalseJump(Label),
    Call(usize, Label),
    Ret,
    FrameCreate(usize),
    FrameRemove(usize),
    Lookup(VariableLookup),
    Halt,
}

#[derive(Hash, Clone)]
pub struct Label {
    pub name: String,
}

#[derive(Clone)]
pub struct VariableLookup {
    pub depth: usize,
    pub offset: usize,
}

impl fmt::Display for IRInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let padding = "    ";
        match self {
            IRInstruction::Push(value) => write!(f, "{padding}push({value})"),
            IRInstruction::Add => write!(f, "{padding}+"),
            IRInstruction::Divide => write!(f, "{padding}/"),
            IRInstruction::Modulo => write!(f, "{padding}%"),
            IRInstruction::Multiply => write!(f, "{padding}*"),
            IRInstruction::Subtract => write!(f, "{padding}-"),
            IRInstruction::Drop => write!(f, "{padding}drop"),
            IRInstruction::Duplicate => write!(f, "{padding}dup"),
            IRInstruction::Over => write!(f, "{padding}over"),
            IRInstruction::Swap => write!(f, "{padding}swap"),
            IRInstruction::Rotate3 => write!(f, "{padding}rot3"),
            IRInstruction::Print => write!(f, "{padding}print"),
            IRInstruction::Less => write!(f, "{padding}<"),
            IRInstruction::Greater => write!(f, "{padding}>"),
            IRInstruction::LessEqual => write!(f, "{padding}<="),
            IRInstruction::GreaterEqual => write!(f, "{padding}>="),
            IRInstruction::Equal => write!(f, "{padding}=="),
            IRInstruction::NotEqual => write!(f, "{padding}!="),
            IRInstruction::And => write!(f, "{padding}&&"),
            IRInstruction::Or => write!(f, "{padding}||"),
            IRInstruction::Assign => write!(f, "{padding}@"),
            IRInstruction::Read => write!(f, "{padding}!"),
            IRInstruction::Input => write!(f, "i{padding}nput"),
            IRInstruction::Mem => write!(f, "{padding}mem"),
            IRInstruction::Syscall(n) => write!(f, "{padding}syscall({n})"),
            IRInstruction::Nth(n) => write!(f, "{padding}nth({n})"),
            IRInstruction::NthWrite(n) => write!(f, "{padding}nth!({n})"),
            IRInstruction::Union(n) => write!(f, "{padding}union({n})"),
            IRInstruction::DebugPrintStack => write!(f, "{padding}__PRINT_STACK__"),
            IRInstruction::DebugHeapStack => write!(f, "{padding}__PRINT_HEAP__"),
            IRInstruction::Label(label) => write!(f, ".{label}"),
            IRInstruction::Jump(label) => write!(f, "{padding}jump({label})"),
            IRInstruction::CondFalseJump(label) => write!(f, "{padding}cond_false_jump({label})"),
            IRInstruction::Call(args, label) => write!(f, "{padding}call({args}, {label})"),
            IRInstruction::Ret => write!(f, "{padding}ret"),
            IRInstruction::FrameCreate(args) => write!(f, "{padding}frame_create({args})"),
            IRInstruction::FrameRemove(args) => write!(f, "{padding}frame_remove({args})"),
            IRInstruction::Lookup(variable) => write!(f, "{padding}lookup({variable})"),
            IRInstruction::Halt => write!(f, "{padding}halt"),
        }
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for VariableLookup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.depth, self.offset)
    }
}
