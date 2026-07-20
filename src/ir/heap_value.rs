use crate::builtins::types::Type;

use std::fmt;

#[derive(Debug, Clone)]
pub struct HeapValue {
    pub r#type: Type,
    pub len: usize,
    pub data: Box<[u8]>,
}

impl fmt::Display for HeapValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.r#type, self.len)
    }
}
