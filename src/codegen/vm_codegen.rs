use crate::{
    error::codegen_error::CodeGenError, ir::ir_instructions::IRInstruction,
    vm::instructions::Instruction,
};

use std::collections::HashMap;

use super::codegen::CodeGen;

impl CodeGen {
    pub(crate) fn vm_process_instructions(&mut self) -> Result<Vec<Instruction>, CodeGenError> {
        let mut instructions = self.ir.instructions.clone();
        let labels = self.process_labels(&mut instructions);

        Ok(instructions
            .iter()
            .map(|i| self.vm_process_ir_instruction(&i, &labels))
            .collect())
    }

    fn vm_process_ir_instruction(
        &self,
        instruction: &IRInstruction,
        labels: &HashMap<String, usize>,
    ) -> Instruction {
        match instruction {
            IRInstruction::Push(v) => Instruction::Push(v.clone()),

            IRInstruction::Add => Instruction::Add,
            IRInstruction::Divide => Instruction::Divide,
            IRInstruction::Modulo => Instruction::Modulo,
            IRInstruction::Multiply => Instruction::Multiply,
            IRInstruction::Subtract => Instruction::Subtract,

            IRInstruction::Drop => Instruction::Drop,
            IRInstruction::Duplicate => Instruction::Duplicate,
            IRInstruction::Over => Instruction::Over,
            IRInstruction::Swap => Instruction::Swap,
            IRInstruction::Rotate3 => Instruction::Rotate3,
            IRInstruction::Print => Instruction::Print,

            IRInstruction::Less => Instruction::Less,
            IRInstruction::Greater => Instruction::Greater,
            IRInstruction::LessEqual => Instruction::LessEqual,
            IRInstruction::GreaterEqual => Instruction::GreaterEqual,
            IRInstruction::Equal => Instruction::Equal,
            IRInstruction::NotEqual => Instruction::NotEqual,
            IRInstruction::And => Instruction::And,
            IRInstruction::Or => Instruction::Or,

            IRInstruction::Assign => Instruction::Assign,
            IRInstruction::Read => Instruction::Read,

            IRInstruction::Input => Instruction::Input,

            IRInstruction::Mem => Instruction::Mem,
            IRInstruction::Syscall(n) => Instruction::Syscall(*n),

            IRInstruction::Nth(n) => Instruction::Nth(*n),
            IRInstruction::NthWrite(n) => Instruction::NthWrite(*n),
            IRInstruction::Union(n) => Instruction::Union(*n),

            IRInstruction::DebugPrintStack => Instruction::DebugPrintStack,
            IRInstruction::DebugHeapStack => Instruction::DebugHeapStack,

            IRInstruction::Label(_) => unreachable!(),
            IRInstruction::Jump(label) => Instruction::Jump(*labels.get(&label.name).unwrap()),
            IRInstruction::CondFalseJump(label) => {
                Instruction::CondFalseJump(*labels.get(&label.name).unwrap())
            }
            IRInstruction::Call(args, label) => {
                Instruction::Call(*args, *labels.get(&label.name).unwrap())
            }
            IRInstruction::Ret => Instruction::Ret,
            IRInstruction::FrameCreate(args) => Instruction::FrameCreate(*args),
            IRInstruction::FrameRemove(args) => Instruction::FrameRemove(*args),
            IRInstruction::Lookup(lookup) => Instruction::Lookup(lookup.depth, lookup.offset),
            IRInstruction::Halt => Instruction::Halt,
        }
    }
}
