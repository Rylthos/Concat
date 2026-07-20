use crate::vm::instructions::Instruction;

use super::codegen::CodeGen;

impl CodeGen {
    pub(crate) fn print(&self, instructions: &Vec<Instruction>) {
        println!("{:?}", instructions);
    }
}
