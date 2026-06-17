use crate::lexer::tokens;
use std::collections::HashMap;

use crate::config::config::Config;

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

    pub fn lex_input(&mut self) -> Result<(), String> {
        self.tokens = self.scan_tokens()?;

        return Ok(());
    }

    pub fn scan_tokens(&self) -> Result<Vec<Token>, String> {
        let mut chars = self.input.chars().peekable();

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

        while let Some(&c) = chars.peek() {
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
                    if let Some(&c2) = chars.peek() {
                        if c2 == '/' {
                            passing_comments = true;
                            continue;
                        }
                    }

                    tokens.push(Token::new(TokenType::Divide, line_number))
                }
                '&' => {
                    chars.next();
                    if let Some(&c2) = chars.peek()
                        && c2 == '&'
                    {
                        tokens.push(Token::new(TokenType::And, line_number));
                        chars.next();
                    } else {
                        panic!("Unknown character");
                    }
                }
                '|' => {
                    chars.next();
                    if let Some(&c2) = chars.peek()
                        && c2 == '|'
                    {
                        tokens.push(Token::new(TokenType::Or, line_number));
                        chars.next();
                    } else {
                        panic!("Unknown character");
                    }
                }
                '<' | '>' | '=' | '!' => {
                    chars.next();
                    let token_type = if let Some(&c2) = chars.peek()
                        && c2 == '='
                    {
                        chars.next();
                        match c {
                            '<' => TokenType::LessEqual,
                            '>' => TokenType::GreaterEqual,
                            '=' => TokenType::Equal,
                            '!' => TokenType::NotEqual,
                            _ => panic!("Unhandled character"),
                        }
                    } else {
                        match c {
                            '<' => TokenType::Less,
                            '>' => TokenType::Greater,
                            '=' => todo!("Assignment"),
                            '!' => TokenType::Not,
                            _ => panic!("Unhandled character"),
                        }
                    };
                    tokens.push(Token::new(token_type, line_number));
                }
                '+' | '*' | '%' | '{' | '}' => {
                    let token = match c {
                        '+' => TokenType::Add,
                        '*' => TokenType::Multiply,
                        '%' => TokenType::Modulo,
                        '{' => TokenType::LeftBrace,
                        '}' => TokenType::RightBrace,
                        _ => panic!("Unaccounted symbol"),
                    };
                    tokens.push(Token::new(token, line_number));
                    chars.next();
                }
                '-' => {
                    chars.next();
                    if let Some(&c2) = chars.peek()
                        && c2 == '>'
                    {
                        tokens.push(Token::new(TokenType::Arrow, line_number));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenType::Subtract, line_number));
                    }
                }
                '"' => {
                    let mut s = String::new();

                    chars.next();
                    while let Some(&c2) = chars.peek() {
                        if c2 == '"' {
                            chars.next();
                            break;
                        } else if c2 == '\\' {
                            chars.next();
                            if let Some(&c3) = chars.peek() {
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

                    tokens.push(Token::new(TokenType::StringValue(s), line_number))
                }
                '0'..='9' => {
                    let mut s = String::new();
                    s.push(c);

                    chars.next();
                    while let Some(&c2) = chars.peek() {
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
                    ))
                }
                '_' | 'A'..='Z' | 'a'..='z' => {
                    let mut s = String::new();
                    s.push(c);

                    chars.next();
                    while let Some(&c2) = chars.peek() {
                        if !c2.is_digit(10) && !c2.is_alphabetic() && !(c2 == '_') {
                            break;
                        } else {
                            s.push(c2);
                            chars.next();
                        }
                    }

                    match keywords.get(&s) {
                        Some(t) => tokens.push(Token::new(t.clone(), line_number)),
                        None => {
                            tokens.push(Token::new(TokenType::Identifier(s), line_number));
                        }
                    }
                }
                _ => panic!("Unhandled Character: {}", c),
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
            Ok(r) => assert_eq!(
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
            Token::new(TokenType::Add, 1),
            Token::new(TokenType::Subtract, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::RightBrace, 1),
            Token::new(TokenType::Multiply, 1),
            Token::new(TokenType::Divide, 1),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_keywords() {
        let input = "string i32 void bool print true false \"Hello, World!\"";
        let output = vec![
            Token::new(TokenType::Type(Types::String), 1),
            Token::new(TokenType::Type(Types::I32), 1),
            Token::new(TokenType::Type(Types::Void), 1),
            Token::new(TokenType::Type(Types::Bool), 1),
            Token::new(TokenType::Print, 1),
            Token::new(TokenType::BoolValue(true), 1),
            Token::new(TokenType::BoolValue(false), 1),
            Token::new(TokenType::StringValue("Hello, World!".to_string()), 1),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_numbers() {
        let input = "0 10 1234";
        let output = vec![
            Token::new(TokenType::I32(0), 1),
            Token::new(TokenType::I32(10), 1),
            Token::new(TokenType::I32(1234), 1),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_escape_lines() {
        let input = r#" "\n \t \" " "#;
        let output = vec![Token::new(
            TokenType::StringValue("\n \t \" ".to_string()),
            1,
        )];
        test_input(input, &output);
    }

    #[test]
    fn lex_comments() {
        let input = "i32 // Hello World\n i32";
        let output = vec![
            Token::new(TokenType::Type(Types::I32), 1),
            Token::new(TokenType::Type(Types::I32), 2),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_arithmetic() {
        let input = "\n1 2 +\n3 4 +\n*\nprint";
        let output = vec![
            Token::new(TokenType::I32(1), 2),
            Token::new(TokenType::I32(2), 2),
            Token::new(TokenType::Add, 2),
            Token::new(TokenType::I32(3), 3),
            Token::new(TokenType::I32(4), 3),
            Token::new(TokenType::Add, 3),
            Token::new(TokenType::Multiply, 4),
            Token::new(TokenType::Print, 5),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_boolean() {
        let input = r#"> < == != <= >="#;
        let output = vec![
            Token::new(TokenType::Greater, 1),
            Token::new(TokenType::Less, 1),
            Token::new(TokenType::Equal, 1),
            Token::new(TokenType::NotEqual, 1),
            Token::new(TokenType::LessEqual, 1),
            Token::new(TokenType::GreaterEqual, 1),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_stack_operations() {
        let input = r#"rot3 dup drop over swap print"#;
        let output = vec![
            Token::new(TokenType::Rotate3, 1),
            Token::new(TokenType::Duplicate, 1),
            Token::new(TokenType::Drop, 1),
            Token::new(TokenType::Over, 1),
            Token::new(TokenType::Swap, 1),
            Token::new(TokenType::Print, 1),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_while_loop() {
        let input = r#"0 while dup 1 > {1 +}"#;
        let output = vec![
            Token::new(TokenType::I32(0), 1),
            Token::new(TokenType::While, 1),
            Token::new(TokenType::Duplicate, 1),
            Token::new(TokenType::I32(1), 1),
            Token::new(TokenType::Greater, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::I32(1), 1),
            Token::new(TokenType::Add, 1),
            Token::new(TokenType::RightBrace, 1),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_if() {
        let input = r#"0 if 1 > { "Less\n" print } else { "Greater\n" print }"#;
        let output = vec![
            Token::new(TokenType::I32(0), 1),
            Token::new(TokenType::If, 1),
            Token::new(TokenType::I32(1), 1),
            Token::new(TokenType::Greater, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::StringValue("Less\n".to_string()), 1),
            Token::new(TokenType::Print, 1),
            Token::new(TokenType::RightBrace, 1),
            Token::new(TokenType::Else, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::StringValue("Greater\n".to_string()), 1),
            Token::new(TokenType::Print, 1),
            Token::new(TokenType::RightBrace, 1),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_func() {
        let input = r#"func test i32 i32 -> i32 { + }"#;
        let output = vec![
            Token::new(TokenType::Func, 1),
            Token::new(TokenType::Identifier("test".to_string()), 1),
            Token::new(TokenType::Type(Types::I32), 1),
            Token::new(TokenType::Type(Types::I32), 1),
            Token::new(TokenType::Arrow, 1),
            Token::new(TokenType::Type(Types::I32), 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::Add, 1),
            Token::new(TokenType::RightBrace, 1),
        ];
        test_input(input, &output);
    }
}
