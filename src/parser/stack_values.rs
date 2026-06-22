use crate::parser::stack_types::StackType;

#[derive(Debug, Clone)]
pub enum StackValue {
    String(String),
    I32(i32),
    Bool(bool),
    Type(StackType),
    Ptr(usize),
    Frame(isize),
    Call(usize),
    VarRef(usize, usize),
}
