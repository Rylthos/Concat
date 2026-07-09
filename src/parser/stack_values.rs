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
    Union(Vec<Box<StackValue>>),
}

impl StackValue {
    pub fn from_type(stack_type: &StackType) -> StackValue {
        match stack_type {
            StackType::I32 => StackValue::I32(0),
            StackType::Bool => StackValue::Bool(false),
            StackType::Char => StackValue::Char(0 as char),
            StackType::Var(_) => unreachable!(),
            StackType::Ptr(is_const, _) => StackValue::Pointer(PointerValue {
                allocation: 0,
                constant: *is_const,
                offset: 0,
            }),
            StackType::Union(entries) => StackValue::Union(
                entries
                    .iter()
                    .map(|e| Box::new(StackValue::from_type(e)))
                    .collect(),
            ),
            StackType::RecordIden(_) => todo!(),
        }
    }
}
