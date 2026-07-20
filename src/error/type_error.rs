use crate::{builtins::types::Type, lexer::tokens::PositionInfo};

#[derive(Debug)]
pub enum TypeError {
    InvalidStackSize(PositionInfo, usize, usize),
    CannotConvertTypeTo(PositionInfo, Type, Type),
    FunctionOutputInvalid(PositionInfo, Vec<Type>, Vec<Type>),
    UnknownLiteral(PositionInfo, String),
    UnknownRecord(PositionInfo, String),
    ExpectedRecordIdenGot(PositionInfo, Type),
    InvalidRecordIdentifier(PositionInfo, String, String),
    InvalidAdd(PositionInfo, Type),
    ExpectedTypeGot(PositionInfo, Type, Type),
    InvalidReadWrite(PositionInfo, Type),
    ExpectedTypeTypeGot(PositionInfo, Type),
    InvalidSize(PositionInfo, usize, usize),
}

impl TypeError {
    pub fn print(&self) {
        match self {
            TypeError::InvalidStackSize(pos, got, max) => {
                eprintln!("[TYPE] [{}] Invalid stack size {}/{}", pos, got, max)
            }
            TypeError::CannotConvertTypeTo(pos, t1, t2) => {
                eprintln!("[TYPE] [{}] Cannot convert from {} to {}", pos, t1, t2)
            }
            TypeError::FunctionOutputInvalid(pos, stack, target) => {}
            TypeError::UnknownLiteral(pos, s) => {
                eprintln!("[TYPE] [{}] Unknown literal {:?}", pos, s)
            }
            TypeError::UnknownRecord(pos, s) => {
                eprintln!("[TYPE] [{}] Unknown record {:?}", pos, s)
            }
            TypeError::ExpectedRecordIdenGot(pos, t) => {
                eprintln!("[TYPE] [{}] Expected record got {:?}", pos, t)
            }
            TypeError::InvalidRecordIdentifier(pos, record, identifier) => {
                eprintln!(
                    "[TYPE] [{}] Invalid record identifier {}.{}",
                    pos, record, identifier
                )
            }
            TypeError::InvalidAdd(pos, t) => {
                eprintln!("[TYPE] [{}] Expected Add or Ptr got {}", pos, t)
            }
            TypeError::ExpectedTypeGot(pos, t1, t2) => {
                eprintln!("[TYPE] [{}] Expected type {} got {}", pos, t1, t2)
            }
            TypeError::InvalidReadWrite(pos, t) => {
                eprintln!("[TYPE] [{}] Expected Var or Ptr got {}", pos, t)
            }
            TypeError::ExpectedTypeTypeGot(pos, t) => {
                eprintln!("[TYPE] [{}] Expected Type got {}", pos, t)
            }
            TypeError::InvalidSize(pos, n, max) => {
                eprintln!("[TYPE] [{}] Invalid size {}/{}", pos, n, max)
            }
        }
    }
}
