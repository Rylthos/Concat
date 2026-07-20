use crate::{
    config::config::Config,
    error::codegen_error::CodeGenError,
    ir::{
        heap_value::HeapValue,
        ir::IRData,
        ir_instructions::{IRInstruction, Label},
    },
    vm::{instructions::Instruction, vm::VMData},
};

use std::collections::HashMap;

pub struct CodeGen {
    config: Config,

    pub(crate) ir: IRData,

    pub(crate) labels: HashMap<Label, usize>,
}

impl CodeGen {
    pub fn init(config: Config, ir: IRData) -> CodeGen {
        CodeGen {
            config,
            ir,
            labels: HashMap::new(),
        }
    }

    pub fn generate_vm(&mut self) -> Result<VMData, CodeGenError> {
        Ok(VMData {
            instructions: self.vm_process_instructions()?,
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
