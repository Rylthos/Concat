#[derive(Debug)]
pub enum LexerError {
    InvalidToken(usize, String),
    ExpectedCharacter(usize),
    InvalidCharacter(usize, char),
}

#[derive(Debug)]
pub enum ErrorType {
    Lexer(LexerError),
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

pub fn print_error(error: ErrorType) {
    match error {
        ErrorType::Lexer(err) => handle_lexer_error(err),
    }
}
