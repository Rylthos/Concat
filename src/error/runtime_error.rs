use syscalls::Errno;

use crate::{ir::stack_values::StackValue, vm::instructions::Instruction};

#[derive(Debug)]
pub enum RuntimeError {
    SyscallInvalidType(Instruction, StackValue),
    SyscallError(Instruction, Errno),
}

impl RuntimeError {
    pub fn print(&self) {
        match self {
            Self::SyscallInvalidType(instr, value) => {
                eprintln!("[RUNTIME] [{:?}] {value} is invalid", instr);
            }
            Self::SyscallError(instr, value) => {
                eprintln!("[RUNTIME] [{:?}] {value}", instr);
            }
        }
    }
}
