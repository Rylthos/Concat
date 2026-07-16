use crate::parser::parse_tree::{FuncDecl, RecordDecl};
use crate::parser::stack_values::StackValue;

use std::collections::HashMap;

pub struct ParseInfo {
    pub functions: HashMap<String, FuncDecl>,
    pub records: HashMap<String, RecordDecl>,
    pub constants: HashMap<String, StackValue>,
}

impl ParseInfo {
    pub fn new() -> ParseInfo {
        ParseInfo {
            functions: HashMap::new(),
            records: HashMap::new(),
            constants: HashMap::new(),
        }
    }
}
