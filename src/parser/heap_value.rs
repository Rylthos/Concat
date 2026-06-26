use crate::parser::stack_types::StackType;

#[derive(Debug, Clone)]
pub struct HeapValue {
    pub r#type: StackType,
    pub len: usize,
    pub data: Box<[u8]>,
}
