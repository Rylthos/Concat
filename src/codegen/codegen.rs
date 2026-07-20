use crate::{
    config::config::Config,
    error::codegen_error::CodeGenError,
    ir::{ir::IRData, ir_instructions::IRInstruction},
    vm::vm::VMData,
};

use std::collections::HashMap;

pub struct CodeGen {
    config: Config,

    pub(crate) ir: IRData,
}

impl CodeGen {
    pub fn init(config: Config, ir: IRData) -> CodeGen {
        CodeGen { config, ir }
    }

    pub fn generate_vm(&mut self) -> Result<VMData, CodeGenError> {
        let instructions = self.vm_process_instructions()?;

        if self.config.codegen_print {
            self.print(&instructions);
        }

        Ok(VMData {
            instructions: instructions,
            initial_heap: self.ir.initial_heap.clone(),
        })
    }

    pub(crate) fn process_labels(
        &mut self,
        instructions: &mut Vec<IRInstruction>,
    ) -> HashMap<String, usize> {
        let mut index: usize = 0;
        let mut labels = HashMap::new();

        while index < instructions.len() {
            let instruction = instructions.get(index).unwrap();

            match instruction {
                IRInstruction::Label(label) => {
                    labels.insert(label.name.clone(), index);
                    instructions.remove(index);
                }
                _ => index += 1,
            }
        }

        labels
    }
}
