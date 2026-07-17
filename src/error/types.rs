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
    CircularInclude(String),
}

#[derive(Debug)]
pub enum ParserError {
    InvalidFunctionDef(PositionInfo, TokenType),
    InvalidRecordDef(PositionInfo, TokenType),
    ExpectedToken(PositionInfo, TokenType),
    ExpectedTokenGot(PositionInfo, TokenType, TokenType),
    ExpectedIdentifierGot(PositionInfo, TokenType),
    InvalidDefine(PositionInfo),
    ExpectedIntrinsic(PositionInfo, Intrinsic),
    ExpectedIntrinsicGot(PositionInfo, Intrinsic, Intrinsic),
    ExpectedTypeGot(PositionInfo, TokenType),
    ExpectedPointerGot(PositionInfo, StackType),
    InvalidParseTree(),
    UnknownIdentifier(PositionInfo, String),
    DuplicateFunction(PositionInfo, PositionInfo, String),
    DuplicateDefine(PositionInfo, PositionInfo, String),
    DuplicateRecord(PositionInfo, PositionInfo, String),
    DuplicateRecordEntry(PositionInfo, String, String),
    ExpectedIntConstant(PositionInfo),
}

#[derive(Debug)]
pub enum TypeError {
    InvalidType(PositionInfo, StackType, StackType),
    InvalidNumberOfArguments(PositionInfo, usize, usize),
    InvalidTypeSet(PositionInfo, HashSet<StackType>, StackType),
    InvalidShape(PositionInfo, Vec<StackType>, Vec<StackType>),
    InvalidIndex(PositionInfo, usize, usize),
    InvalidRecordWriteType(PositionInfo, StackType, StackType),
    InvalidIdentifier(PositionInfo, String),
    InvalidRecordName(PositionInfo, String),
    InvalidRecordEntry(PositionInfo, String, String),
    DuplicateRecordEntry(PositionInfo, String, String),
}

#[derive(Debug)]
pub enum ErrorType {
    Lexer(LexerError),
    Parser(ParserError),
    Type(TypeError),
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
        LexerError::CircularInclude(include) => {
            eprintln!("[LEXER] Previously included file \"{}\"", include)
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
        ParserError::InvalidRecordDef(pos, token) => {
            eprintln!(
                "[PARSER] [{}] Invalid record definition, expected identifier, got {}",
                pos, token
            )
        }
        ParserError::ExpectedToken(pos, token) => {
            eprintln!("[PARSER] [{}] Expected token {}", pos, token);
        }
        ParserError::ExpectedTokenGot(pos, expected, got) => {
            eprintln!("[PARSER] [{}] Expected token {} got {}", pos, expected, got);
        }
        ParserError::ExpectedIdentifierGot(pos, token) => {
            eprintln!("[PARSER] [{}] Expected identifier got {}", pos, token);
        }
        ParserError::InvalidDefine(pos) => {
            eprintln!("[PARSER] [{}] Invalid definition", pos);
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
        ParserError::DuplicateFunction(pos, previous, name) => {
            eprintln!(
                "[PARSER] [{}] Duplicate function definition {}, previously defined at {}",
                pos, name, previous
            );
        }
        ParserError::DuplicateDefine(pos, previous, name) => {
            eprintln!(
                "[PARSER] [{}] Duplicate define {}, previously defined at {}",
                pos, name, previous
            );
        }
        ParserError::DuplicateRecord(pos, previous, name) => {
            eprintln!(
                "[PARSER] [{}] Duplicate record {}, previously defined at {}",
                pos, name, previous
            );
        }
        ParserError::DuplicateRecordEntry(pos, record, entry) => {
            eprintln!(
                "[PARSER] [{}] Duplicate record entry {}.{}",
                pos, record, entry
            );
        }
        ParserError::ExpectedIntConstant(pos) => {
            eprintln!("[PARSER] [{}] Expected Int constant", pos);
        }
    }
}

fn handle_type_error(error: TypeError) {
    match error {
        TypeError::InvalidNumberOfArguments(pos, expected, got) => {
            eprintln!(
                "[TYPE] [{}] Expected {} arguments, got {}",
                pos, expected, got
            );
        }
        TypeError::InvalidType(pos, input, output) => {
            eprintln!("[TYPE] [{}] Expected {} type, got {}", pos, input, output);
        }
        TypeError::InvalidTypeSet(pos, inputs, output) => {
            eprintln!(
                "[TYPE] [{}] Expected one of {:?}, got {}",
                pos, inputs, output
            );
        }
        TypeError::InvalidShape(pos, stack1, stack2) => {
            eprintln!(
                "[TYPE] [{}] Stack shapes differ {:?} {:?}",
                pos, stack1, stack2
            );
        }

        TypeError::InvalidIndex(pos, index, size) => {
            eprintln!("[TYPE] [{}] Invalid index {}/{}", pos, index, size);
        }
        TypeError::InvalidRecordWriteType(pos, expected, got) => {
            eprintln!(
                "[TYPE] [{}] Invalid record write type, got {} expected {}",
                pos, got, expected
            );
        }

        TypeError::InvalidIdentifier(pos, identifier) => {
            eprintln!("[TYPE] [{}] Invalid Identifier {}", pos, identifier);
        }
        TypeError::InvalidRecordName(pos, record) => {
            eprintln!("[TYPE] [{}] Invalid record name {}", pos, record);
        }
        TypeError::InvalidRecordEntry(pos, record, entry) => {
            eprintln!("[TYPE] [{}] Invalid record entry {}.{}", pos, record, entry);
        }
        TypeError::DuplicateRecordEntry(pos, record, entry) => {
            eprintln!(
                "[TYPE] [{}] Duplicate record entry {}.{}",
                pos, record, entry
            );
        }
    }
}

pub fn print_error(error: ErrorType) {
    match error {
        ErrorType::Lexer(err) => handle_lexer_error(err),
        ErrorType::Parser(err) => handle_parser_error(err),
        ErrorType::Type(err) => handle_type_error(err),
    }
}
