use crate::lexer::tokens::{PositionInfo, TokenType};
use crate::parser::stack_types::StackType;

#[derive(Debug)]
pub enum LexerError {
    InvalidToken(PositionInfo, String),
    ExpectedCharacter(PositionInfo),
    InvalidCharacter(PositionInfo, char),
}

#[derive(Debug)]
pub enum ParserError {
    InvalidFunctionDef(PositionInfo, TokenType),
    ExpectedToken(PositionInfo, TokenType),
    ExpectedTokenGot(PositionInfo, TokenType, TokenType),
    InvalidParseTree(),
    UnknownIdentifier(PositionInfo, String),
    InvalidNumberOfArguments(PositionInfo, usize, usize),
    InvalidType(PositionInfo, StackType, StackType),
    InvalidShape(PositionInfo),
}

#[derive(Debug)]
pub enum ErrorType {
    Lexer(LexerError),
    Parser(ParserError),
}

fn handle_lexer_error(error: LexerError) {
    match error {
        LexerError::InvalidToken(pos, token) => {
            eprintln!(
                "[LEXER] [{}:{}] Invalid token {}",
                pos.line, pos.column, token
            );
        }
        LexerError::ExpectedCharacter(pos) => {
            eprintln!("[LEXER] [{}:{}] Expected character", pos.line, pos.column);
        }
        LexerError::InvalidCharacter(pos, c) => {
            eprintln!(
                "[LEXER] [{}:{}] Invalid character {}",
                pos.line, pos.column, c
            );
        }
    }
}

fn handle_parser_error(error: ParserError) {
    match error {
        ParserError::InvalidFunctionDef(pos, token) => {
            eprintln!(
                "[PARSER] [{}:{}] Invalid function definition, expected identifier, got {:?}",
                pos.line, pos.column, token
            )
        }
        ParserError::ExpectedToken(pos, token) => {
            eprintln!(
                "[PARSER] [{}:{}] Expected token {:?}",
                pos.line, pos.column, token
            );
        }
        ParserError::ExpectedTokenGot(pos, expected, got) => {
            eprintln!(
                "[PARSER] [{}:{}] Expected token {:?}, got {:?}",
                pos.line, pos.column, expected, got
            );
        }
        ParserError::InvalidParseTree() => {
            eprintln!("[PARSER]: Invalid parse tree");
        }
        ParserError::UnknownIdentifier(pos, name) => {
            eprintln!(
                "[PARSER] [{}:{}] Unknown identifier {}",
                pos.line, pos.column, name
            );
        }
        ParserError::InvalidNumberOfArguments(pos, expected, got) => {
            eprintln!(
                "[TYPE] [{}:{}] Expected {} arguments, got {}",
                pos.line, pos.column, expected, got
            );
        }
        ParserError::InvalidType(pos, input, output) => {
            eprintln!(
                "[TYPE] [{}:{}] Expected {} arguments, got {}",
                pos.line, pos.column, input, output
            );
        }
        ParserError::InvalidShape(pos) => {
            eprintln!("[TYPE] [{}:{}] Stack shapes differ", pos.line, pos.column);
        }
    }
}

pub fn print_error(error: ErrorType) {
    match error {
        ErrorType::Lexer(err) => handle_lexer_error(err),
        ErrorType::Parser(err) => handle_parser_error(err),
    }
}
