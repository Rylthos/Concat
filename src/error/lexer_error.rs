use crate::lexer::tokens::{PositionInfo, Token};

#[derive(Debug, Clone)]
pub enum LexerError {
    InvalidToken(PositionInfo, String),
    ExpectedCharacter(PositionInfo),
    ExpectedCharacterGot(PositionInfo, char, char),
    InvalidCharacter(PositionInfo, char),
    InvalidFile(String),
    ExpectedFilePath(PositionInfo),
    InvalidInclude(PositionInfo, Token),
    CircularInclude(String),
}

impl LexerError {
    pub fn print(&self) {
        match self {
            LexerError::InvalidToken(pos, token) => {
                eprintln!("[LEXER] [{}] Invalid token {}", pos, token);
            }
            LexerError::ExpectedCharacter(pos) => {
                eprintln!("[LEXER] [{}] Expected character", pos);
            }
            LexerError::ExpectedCharacterGot(pos, got, expected) => {
                eprintln!(
                    "[LEXER] [{}] Expected character {}, got {}",
                    pos, got, expected
                );
            }
            LexerError::InvalidCharacter(pos, c) => {
                eprintln!("[LEXER] [{}] Invalid character {}", pos, c);
            }
            LexerError::InvalidFile(file) => {
                eprintln!("[LEXER] Invalid file \"{}\"", file);
            }
            LexerError::ExpectedFilePath(pos) => {
                eprintln!("[LEXER] [{}] Expected file path", pos);
            }
            LexerError::InvalidInclude(pos, token) => {
                eprintln!(
                    "[LEXER] [{}] Expected file path, got {}:{}",
                    pos, token.token_type, token.string
                );
            }
            LexerError::CircularInclude(include) => {
                eprintln!("[LEXER] Previously included file \"{}\"", include)
            }
        }
    }
}
