use crate::ir::ir_instructions::IRInstruction;

use super::ir::IR;

impl IR {
    pub(crate) fn print(&self, instructions: &Vec<IRInstruction>) {
        println!("===== IR =====");

        println!("===== HEAP =====");
        for (i, h) in (0..).zip(self.heap.iter()) {
            println!("{i}: {}", h);
        }
        println!("===== HEAP =====");

        println!("===== INSTR =====");
        for i in instructions {
            println!("{}", i)
        }
        println!("===== INSTR =====");

        println!("===== IR =====");
    }
}
