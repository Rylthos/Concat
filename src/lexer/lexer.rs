use crate::lexer::tokens::{self, PositionInfo};
use std::collections::HashMap;

use crate::config::config::Config;
use crate::error::types::{ErrorType, LexerError};

use tokens::{Token, TokenType, Types};

pub struct Lexer {
    config: Config,
    input: String,

    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn init(config: Config, input: String) -> Lexer {
        Lexer {
            config,
            input,
            tokens: Vec::new(),
        }
    }

    pub fn lex_input(&mut self) -> Result<(), ErrorType> {
        self.tokens = match self.scan_tokens() {
            Ok(t) => t,
            Err(err) => return Err(ErrorType::Lexer(err)),
        };

        if self.config.token_print {
            println!("=== TOKENS ===");
            for token in self.tokens.iter() {
                println!("{}", token);
            }
            println!("=== TOKENS ===");
        }

        return Ok(());
    }

    pub fn scan_tokens(&self) -> Result<Vec<Token>, LexerError> {
        let raw_chars = self.input.chars();

        let mut matched_chars: Vec<(usize, char)> = Vec::new();
        let mut column = 1;
        for c in raw_chars {
            matched_chars.push((column, c));
            column += 1;
            match c {
                '\n' => column = 1,
                _ => (),
            }
        }
        let mut chars = matched_chars.iter().cloned().peekable();

        let mut tokens: Vec<Token> = Vec::new();

        let mut passing_comments: bool = false;

        let mut line_number: usize = 1;

        let keywords = HashMap::from([
            ("true".to_string(), TokenType::BoolValue(true)),
            ("false".to_string(), TokenType::BoolValue(false)),
            //
            ("string".to_string(), TokenType::Type(Types::String)),
            ("bool".to_string(), TokenType::Type(Types::Bool)),
            ("i32".to_string(), TokenType::Type(Types::I32)),
            ("void".to_string(), TokenType::Type(Types::Void)),
            //
            ("rot3".to_string(), TokenType::Rotate3),
            ("dup".to_string(), TokenType::Duplicate),
            ("drop".to_string(), TokenType::Drop),
            ("over".to_string(), TokenType::Over),
            ("swap".to_string(), TokenType::Swap),
            ("print".to_string(), TokenType::Print),
            //
            ("if".to_string(), TokenType::If),
            ("else".to_string(), TokenType::Else),
            ("while".to_string(), TokenType::While),
            //
            ("func".to_string(), TokenType::Func),
        ]);

        while let Some(&(column_number, c)) = chars.peek() {
            if passing_comments {
                if c == '\n' {
                    passing_comments = false
                } else {
                    chars.next();
                    continue;
                }
            }

            match c {
                ' ' | '\t' => {
                    chars.next();
                }
                '\n' => {
                    line_number += 1;
                    chars.next();
                }
                '/' => {
                    chars.next();
                    if let Some(&(_, c2)) = chars.peek() {
                        if c2 == '/' {
                            passing_comments = true;
                            continue;
                        }
                    }

                    tokens.push(Token::new(
                        TokenType::Divide,
                        line_number,
                        column_number,
                        "/",
                    ))
                }
                '&' => {
                    chars.next();
                    if let Some(&(_, c2)) = chars.peek() {
                        if c2 == '&' {
                            tokens.push(Token::new(
                                TokenType::And,
                                line_number,
                                column_number,
                                "&&",
                            ));
                            chars.next();
                        } else {
                            return Err(LexerError::InvalidToken(
                                PositionInfo {
                                    line: line_number,
                                    column: column_number,
                                    string: "".to_string(),
                                },
                                format!("&{:?}", c2),
                            ));
                        }
                    } else {
                        return Err(LexerError::ExpectedCharacter(PositionInfo {
                            line: line_number,
                            column: column_number,
                            string: "".to_string(),
                        }));
                    }
                }
                '|' => {
                    chars.next();
                    if let Some(&(_, c2)) = chars.peek() {
                        if c2 == '|' {
                            tokens.push(Token::new(
                                TokenType::Or,
                                line_number,
                                column_number,
                                "||",
                            ));
                            chars.next();
                        } else {
                            return Err(LexerError::InvalidToken(
                                PositionInfo {
                                    line: line_number,
                                    column: column_number,
                                    string: "".to_string(),
                                },
                                format!("|{:?}", c2),
                            ));
                        }
                    } else {
                        return Err(LexerError::ExpectedCharacter(PositionInfo {
                            line: line_number,
                            column: column_number,
                            string: "".to_string(),
                        }));
                    }
                }
                '<' | '>' | '=' | '!' => {
                    chars.next();
                    let lexed_string: String;
                    let token_type = if let Some(&(_, c2)) = chars.peek()
                        && c2 == '='
                    {
                        chars.next();
                        lexed_string = format!("{}=", c);
                        match c {
                            '<' => TokenType::LessEqual,
                            '>' => TokenType::GreaterEqual,
                            '=' => TokenType::Equal,
                            '!' => TokenType::NotEqual,
                            _ => unreachable!("Unhandled case"),
                        }
                    } else {
                        lexed_string = format!("{}", c);
                        match c {
                            '<' => TokenType::Less,
                            '>' => TokenType::Greater,
                            '=' => todo!(),
                            '!' => TokenType::Not,
                            _ => unreachable!("Unhandled case"),
                        }
                    };
                    tokens.push(Token::new(
                        token_type,
                        line_number,
                        column_number,
                        &lexed_string,
                    ));
                }
                '+' | '*' | '%' | '{' | '}' => {
                    let token = match c {
                        '+' => TokenType::Add,
                        '*' => TokenType::Multiply,
                        '%' => TokenType::Modulo,
                        '{' => TokenType::LeftBrace,
                        '}' => TokenType::RightBrace,
                        _ => unreachable!("Unhandled case"),
                    };
                    tokens.push(Token::new(
                        token,
                        line_number,
                        column_number,
                        &format!("{}", c),
                    ));
                    chars.next();
                }
                '-' => {
                    chars.next();
                    if let Some(&(_, c2)) = chars.peek()
                        && c2 == '>'
                    {
                        tokens.push(Token::new(
                            TokenType::Arrow,
                            line_number,
                            column_number,
                            "->",
                        ));
                        chars.next();
                    } else {
                        tokens.push(Token::new(
                            TokenType::Subtract,
                            line_number,
                            column_number,
                            "-",
                        ));
                    }
                }
                '"' => {
                    let mut s = String::new();

                    chars.next();
                    while let Some(&(_, c2)) = chars.peek() {
                        if c2 == '"' {
                            chars.next();
                            break;
                        } else if c2 == '\\' {
                            chars.next();
                            if let Some(&(_, c3)) = chars.peek() {
                                match c3 {
                                    'n' => s.push('\n'),
                                    't' => s.push('\t'),
                                    '\\' => s.push('\\'),
                                    '\"' => s.push('\"'),
                                    _ => (),
                                }
                            }
                            chars.next();
                        } else {
                            s.push(c2);
                            chars.next();
                        }
                    }

                    tokens.push(Token::new(
                        TokenType::StringValue(s.clone()),
                        line_number,
                        column_number,
                        &format!("{:?}", s),
                    ))
                }
                '0'..='9' => {
                    let mut s = String::new();
                    s.push(c);

                    chars.next();
                    while let Some(&(_, c2)) = chars.peek() {
                        match c2 {
                            '0'..='9' => {
                                s.push(c2);
                                chars.next();
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    tokens.push(Token::new(
                        TokenType::I32(s.parse::<i32>().unwrap()),
                        line_number,
                        column_number,
                        &s,
                    ))
                }
                '_' | 'A'..='Z' | 'a'..='z' => {
                    let mut s = String::new();
                    s.push(c);

                    chars.next();
                    while let Some(&(_, c2)) = chars.peek() {
                        if !c2.is_digit(10) && !c2.is_alphabetic() && !(c2 == '_') {
                            break;
                        } else {
                            s.push(c2);
                            chars.next();
                        }
                    }

                    match keywords.get(&s) {
                        Some(t) => {
                            tokens.push(Token::new(t.clone(), line_number, column_number, &s))
                        }
                        None => {
                            tokens.push(Token::new(
                                TokenType::Identifier(s.clone()),
                                line_number,
                                column_number,
                                &s,
                            ));
                        }
                    }
                }
                _ => {
                    return Err(LexerError::InvalidCharacter(
                        PositionInfo {
                            line: line_number,
                            column: column_number,
                            string: "".to_string(),
                        },
                        c,
                    ));
                }
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input(input: &str, expected_output: &Vec<Token>) {
        let mut lexer = Lexer::init(Config::blank(), String::from(input));
        let result = lexer.lex_input();
        match result {
            Ok(_) => assert_eq!(
                format!("{:?}", expected_output),
                format!("{:?}", lexer.tokens)
            ),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[test]
    fn lex_single_characters() {
        let input = "+ - {} *\t/";
        let output = vec![
            Token::new(TokenType::Add, 1, 1, "+"),
            Token::new(TokenType::Subtract, 1, 3, "-"),
            Token::new(TokenType::LeftBrace, 1, 5, "{"),
            Token::new(TokenType::RightBrace, 1, 6, "}"),
            Token::new(TokenType::Multiply, 1, 8, "*"),
            Token::new(TokenType::Divide, 1, 10, "/"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_keywords() {
        let input = "string i32 void bool print true false \"Hello, World!\"";
        let output = vec![
            Token::new(TokenType::Type(Types::String), 1, 1, "string"),
            Token::new(TokenType::Type(Types::I32), 1, 8, "i32"),
            Token::new(TokenType::Type(Types::Void), 1, 12, "void"),
            Token::new(TokenType::Type(Types::Bool), 1, 17, "bool"),
            Token::new(TokenType::Print, 1, 22, "print"),
            Token::new(TokenType::BoolValue(true), 1, 28, "true"),
            Token::new(TokenType::BoolValue(false), 1, 33, "false"),
            Token::new(
                TokenType::StringValue("Hello, World!".to_string()),
                1,
                39,
                "\"Hello, World!\"",
            ),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_numbers() {
        let input = "0 10 1234";
        let output = vec![
            Token::new(TokenType::I32(0), 1, 1, "0"),
            Token::new(TokenType::I32(10), 1, 3, "10"),
            Token::new(TokenType::I32(1234), 1, 6, "1234"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_escape_lines() {
        let input = r#" "\n \t \" " "#;
        let output = vec![Token::new(
            TokenType::StringValue("\n \t \" ".to_string()),
            1,
            2,
            "\"\\n \\t \\\" \"",
        )];
        test_input(input, &output);
    }

    #[test]
    fn lex_comments() {
        let input = "i32 // Hello World\n i32";
        let output = vec![
            Token::new(TokenType::Type(Types::I32), 1, 1, "i32"),
            Token::new(TokenType::Type(Types::I32), 2, 2, "i32"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_arithmetic() {
        let input = "\n1 2 +\n3 4 +\n*\nprint";
        let output = vec![
            Token::new(TokenType::I32(1), 2, 1, "1"),
            Token::new(TokenType::I32(2), 2, 3, "2"),
            Token::new(TokenType::Add, 2, 5, "+"),
            Token::new(TokenType::I32(3), 3, 1, "3"),
            Token::new(TokenType::I32(4), 3, 3, "4"),
            Token::new(TokenType::Add, 3, 5, "+"),
            Token::new(TokenType::Multiply, 4, 1, "*"),
            Token::new(TokenType::Print, 5, 1, "print"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_boolean() {
        let input = r#"> < == != <= >="#;
        let output = vec![
            Token::new(TokenType::Greater, 1, 1, ">"),
            Token::new(TokenType::Less, 1, 3, "<"),
            Token::new(TokenType::Equal, 1, 5, "=="),
            Token::new(TokenType::NotEqual, 1, 8, "!="),
            Token::new(TokenType::LessEqual, 1, 11, "<="),
            Token::new(TokenType::GreaterEqual, 1, 14, ">="),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_stack_operations() {
        let input = r#"rot3 dup drop over swap print"#;
        let output = vec![
            Token::new(TokenType::Rotate3, 1, 1, "rot3"),
            Token::new(TokenType::Duplicate, 1, 6, "dup"),
            Token::new(TokenType::Drop, 1, 10, "drop"),
            Token::new(TokenType::Over, 1, 15, "over"),
            Token::new(TokenType::Swap, 1, 20, "swap"),
            Token::new(TokenType::Print, 1, 25, "print"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_while_loop() {
        let input = r#"0 while dup 1 > {1 +}"#;
        let output = vec![
            Token::new(TokenType::I32(0), 1, 1, "0"),
            Token::new(TokenType::While, 1, 3, "while"),
            Token::new(TokenType::Duplicate, 1, 9, "dup"),
            Token::new(TokenType::I32(1), 1, 13, "1"),
            Token::new(TokenType::Greater, 1, 15, ">"),
            Token::new(TokenType::LeftBrace, 1, 17, "{"),
            Token::new(TokenType::I32(1), 1, 18, "1"),
            Token::new(TokenType::Add, 1, 20, "+"),
            Token::new(TokenType::RightBrace, 1, 21, "}"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_if() {
        let input = r#"0 if 1 > { "Less\n" print } else { "Greater\n" print }"#;
        let output = vec![
            Token::new(TokenType::I32(0), 1, 1, "0"),
            Token::new(TokenType::If, 1, 3, "if"),
            Token::new(TokenType::I32(1), 1, 6, "1"),
            Token::new(TokenType::Greater, 1, 8, ">"),
            Token::new(TokenType::LeftBrace, 1, 10, "{"),
            Token::new(
                TokenType::StringValue("Less\n".to_string()),
                1,
                12,
                "\"Less\\n\"",
            ),
            Token::new(TokenType::Print, 1, 21, "print"),
            Token::new(TokenType::RightBrace, 1, 27, "}"),
            Token::new(TokenType::Else, 1, 29, "else"),
            Token::new(TokenType::LeftBrace, 1, 34, "{"),
            Token::new(
                TokenType::StringValue("Greater\n".to_string()),
                1,
                36,
                "\"Greater\\n\"",
            ),
            Token::new(TokenType::Print, 1, 48, "print"),
            Token::new(TokenType::RightBrace, 1, 54, "}"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_func() {
        let input = r#"func test i32 i32 -> i32 { + }"#;
        let output = vec![
            Token::new(TokenType::Func, 1, 1, "func"),
            Token::new(TokenType::Identifier("test".to_string()), 1, 6, "test"),
            Token::new(TokenType::Type(Types::I32), 1, 11, "i32"),
            Token::new(TokenType::Type(Types::I32), 1, 15, "i32"),
            Token::new(TokenType::Arrow, 1, 19, "->"),
            Token::new(TokenType::Type(Types::I32), 1, 22, "i32"),
            Token::new(TokenType::LeftBrace, 1, 26, "{"),
            Token::new(TokenType::Add, 1, 28, "+"),
            Token::new(TokenType::RightBrace, 1, 30, "}"),
        ];
        test_input(input, &output);
    }
}
