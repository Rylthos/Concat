use crate::parser::stack_types::StackType;

pub struct HeapValue {
    pub r#type: StackType,
    pub len: usize,
    pub data: Box<[u8]>,
}
