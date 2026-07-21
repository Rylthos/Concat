use crate::lexer::tokens::PositionInfo;

#[derive(Debug)]
pub enum ReducerError {
    ExpectedIntConstant(PositionInfo),
    InvalidDefine(PositionInfo),
    SyscallOutOfRange(PositionInfo, i32),
}

impl ReducerError {
    pub fn print(&self) {
        match self {
            Self::ExpectedIntConstant(pos) => {
                eprintln!("[REDUCER] [{pos}] Expected integer constant")
            }
            Self::InvalidDefine(pos) => {
                eprintln!("[REDUCER] [{pos}] Invalid define")
            }
            Self::SyscallOutOfRange(pos, n) => {
                eprintln!(
                    "[REDUCER] [{pos}] Syscall argument out of range. {n} should the range of 0..=6"
                )
            }
        }
    }
}
