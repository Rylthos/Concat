use crate::{
    builtins::{basic_types::BasicType, builtins::Builtin},
    lexer::tokens::{PositionInfo, Token, TokenType},
};

#[derive(Debug)]
pub enum ParserError {
    ExpectedToken(PositionInfo),
    UnexpectedToken(Token),
    ExpectedTypeGotToken(PositionInfo, Token),
    ExpectedTypeGotBuiltin(PositionInfo, Builtin),
    ExpectedPointerGotType(PositionInfo, BasicType),
    ExpectedIdentifier(PositionInfo),
    InvalidTypeListVoid(PositionInfo),
    ExpectedTokenGot(PositionInfo, TokenType, TokenType),
}

impl ParserError {
    pub fn print(&self) {
        match self {
            ParserError::ExpectedToken(pos) => {
                eprintln!("[PARSER] [{}] Expected token", pos)
            }
            ParserError::UnexpectedToken(token) => eprintln!(
                "[PARSER] [{}] Unexpected token {}",
                token.position_info, token
            ),

            ParserError::ExpectedTypeGotToken(pos, token) => {
                eprintln!("[PARSER] [{}] Expected type, got {}", pos, token)
            }
            ParserError::ExpectedTypeGotBuiltin(pos, builtin) => {
                eprintln!("[PARSER] [{}] Expected type, got {}", pos, builtin)
            }
            ParserError::ExpectedPointerGotType(pos, t) => {
                eprintln!("[PARSER] [{}] Expected pointer type, Got {}", pos, t)
            }
            ParserError::ExpectedIdentifier(pos) => {
                eprintln!("[PARSER] [{}] Expected identifier", pos)
            }
            ParserError::InvalidTypeListVoid(pos) => {
                eprintln!("[PARSER] [{}] Invalid type list after VOID", pos)
            }
            ParserError::ExpectedTokenGot(pos, expected, got) => {
                eprintln!("[PARSER] [{}] Expected token {} got {}", pos, expected, got)
            }
        }
    }
}
