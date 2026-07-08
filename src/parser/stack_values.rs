use crate::parser::stack_types::StackType;

#[derive(Debug, Clone)]
pub struct PointerValue {
    pub allocation: usize,
    pub constant: bool,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub enum StackValue {
    I32(i32),
    Bool(bool),
    Char(char),
    Type(StackType),
    Pointer(PointerValue),
    Frame(isize),
    Call(usize),
    VarRef(usize, usize),
    Record(Vec<Box<StackValue>>),
}
