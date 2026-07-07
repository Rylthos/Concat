use crate::lexer::tokens::{PositionInfo, Token, TokenType};
use crate::parser::intrinsics::Intrinsic;
use crate::parser::stack_types::StackType;

use std::collections::HashSet;

#[derive(Debug)]
pub enum LexerError {
    InvalidToken(PositionInfo, String),
    ExpectedCharacter(PositionInfo),
    ExpectedCharacterGot(PositionInfo, char, char),
    InvalidCharacter(PositionInfo, char),
    InvalidFile(String),
    ExpectedFilePath(PositionInfo),
    InvalidInclude(PositionInfo, Token),
}

#[derive(Debug)]
pub enum ParserError {
    InvalidFunctionDef(PositionInfo, TokenType),
    ExpectedToken(PositionInfo, TokenType),
    ExpectedIdentifierGot(PositionInfo, TokenType),
    ExpectedIntrinsic(PositionInfo, Intrinsic),
    ExpectedIntrinsicGot(PositionInfo, Intrinsic, Intrinsic),
    ExpectedTypeGot(PositionInfo, TokenType),
    ExpectedPointerGot(PositionInfo, StackType),
    InvalidParseTree(),
    UnknownIdentifier(PositionInfo, String),
    InvalidNumberOfArguments(PositionInfo, usize, usize),
    InvalidType(PositionInfo, StackType, StackType),
    InvalidTypeSet(PositionInfo, HashSet<StackType>, StackType),
    InvalidShape(PositionInfo, Vec<StackType>, Vec<StackType>),
}

#[derive(Debug)]
pub enum ErrorType {
    Lexer(LexerError),
    Parser(ParserError),
}

fn handle_lexer_error(error: LexerError) {
    match error {
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
    }
}

fn handle_parser_error(error: ParserError) {
    match error {
        ParserError::InvalidFunctionDef(pos, token) => {
            eprintln!(
                "[PARSER] [{}] Invalid function definition, expected identifier, got {}",
                pos, token
            )
        }
        ParserError::ExpectedToken(pos, token) => {
            eprintln!("[PARSER] [{}] Expected token {}", pos, token);
        }
        ParserError::ExpectedIdentifierGot(pos, token) => {
            eprintln!("[PARSER] [{}] Expected identifier got {}", pos, token);
        }
        ParserError::ExpectedIntrinsic(pos, token) => {
            eprintln!("[PARSER] [{}] Expected intrinsic {}", pos, token);
        }
        ParserError::ExpectedIntrinsicGot(pos, expected, got) => {
            eprintln!(
                "[PARSER] [{}] Expected intrinsic {}, got {}",
                pos, expected, got
            );
        }
        ParserError::ExpectedTypeGot(pos, got) => {
            eprintln!("[PARSER] [{}] Expected type , got {}", pos, got);
        }
        ParserError::ExpectedPointerGot(pos, got) => {
            eprintln!("[PARSER] [{}] Expected pointer , got {}", pos, got);
        }
        ParserError::InvalidParseTree() => {
            eprintln!("[PARSER]: Invalid parse tree");
        }
        ParserError::UnknownIdentifier(pos, name) => {
            eprintln!("[PARSER] [{}] Unknown identifier {}", pos, name);
        }
        ParserError::InvalidNumberOfArguments(pos, expected, got) => {
            eprintln!(
                "[TYPE] [{}] Expected {} arguments, got {}",
                pos, expected, got
            );
        }
        ParserError::InvalidType(pos, input, output) => {
            eprintln!("[TYPE] [{}] Expected {} type, got {}", pos, input, output);
        }
        ParserError::InvalidTypeSet(pos, inputs, output) => {
            eprintln!(
                "[TYPE] [{}] Expected one of {:?}, got {}",
                pos, inputs, output
            );
        }
        ParserError::InvalidShape(pos, stack1, stack2) => {
            eprintln!(
                "[TYPE] [{}] Stack shapes differ {:?} {:?}",
                pos, stack1, stack2
            );
        }
    }
}

pub fn print_error(error: ErrorType) {
    match error {
        ErrorType::Lexer(err) => handle_lexer_error(err),
        ErrorType::Parser(err) => handle_parser_error(err),
    }
}
