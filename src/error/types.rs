use crate::lexer::tokens::{TokenType, Types};

#[derive(Debug)]
pub enum LexerError {
    InvalidToken(usize, String),
    ExpectedCharacter(usize),
    InvalidCharacter(usize, char),
}

#[derive(Debug)]
pub enum ParserError {
    InvalidFunctionDef(usize, TokenType),
    ExpectedToken(usize, TokenType),
    ExpectedTokenGot(usize, TokenType, TokenType),
    InvalidParseTree(),
    UnknownIdentifier(usize, String),
    InvalidNumberOfArguments(usize, usize, usize),
    InvalidType(usize, Types, Types),
}

#[derive(Debug)]
pub enum ErrorType {
    Lexer(LexerError),
    Parser(ParserError),
}

fn handle_lexer_error(error: LexerError) {
    match error {
        LexerError::InvalidToken(line, token) => {
            eprintln!("[LEXER] Line {}: Invalid token {}", line, token);
        }
        LexerError::ExpectedCharacter(line) => {
            eprintln!("[LEXER] Line {}: Expected character", line);
        }
        LexerError::InvalidCharacter(line, c) => {
            eprintln!("[LEXER] Line {}: Invalid character {}", line, c);
        }
    }
}

fn handle_parser_error(error: ParserError) {
    match error {
        ParserError::InvalidFunctionDef(line, token) => {
            eprintln!(
                "[PARSER] Line {}: Invalid function definition, expected identifier, got {:?}",
                line, token
            )
        }
        ParserError::ExpectedToken(line, token) => {
            eprintln!("[PARSER] Line {}: Expected token {:?}", line, token);
        }
        ParserError::ExpectedTokenGot(line, expected, got) => {
            eprintln!(
                "[PARSER] Line {}: Expected token {:?}, got {:?}",
                line, expected, got
            );
        }
        ParserError::InvalidParseTree() => {
            eprintln!("[PARSER]: Invalid parse tree");
        }
        ParserError::UnknownIdentifier(line, name) => {
            eprintln!("[PARSER] Line {}: Unknown identifier {}", line, name);
        }
        ParserError::InvalidNumberOfArguments(line, expected, got) => {
            eprintln!(
                "[TYPE] Line {}: Expected {} arguments, got {}",
                line, expected, got
            );
        }
        ParserError::InvalidType(line, input, output) => {
            eprintln!(
                "[TYPE] Line {}: Expected {} arguments, got {}",
                line, input, output
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
