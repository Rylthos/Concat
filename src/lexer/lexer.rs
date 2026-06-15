use crate::lexer::tokens;
use std::collections::HashMap;

use tokens::{Token, TokenType, Types};

fn scan_tokens(input: &str) -> Result<Vec<Token>, String> {
    let mut chars = input.chars().peekable();

    let mut tokens: Vec<Token> = Vec::new();

    let mut passing_comments: bool = false;

    let mut line_number: usize = 1;

    let keywords = HashMap::from([
        ("true".to_string(), TokenType::BoolValue(true)),
        ("false".to_string(), TokenType::BoolValue(false)),
        //
        ("string".to_string(), TokenType::Type(Types::String)),
        ("bool".to_string(), TokenType::Type(Types::Bool)),
        ("int".to_string(), TokenType::Type(Types::Int)),
        //
        ("rot".to_string(), TokenType::Rotate),
        ("dup".to_string(), TokenType::Duplicate),
        ("drop".to_string(), TokenType::Drop),
        ("over".to_string(), TokenType::Over),
        ("swap".to_string(), TokenType::Swap),
        ("print".to_string(), TokenType::Print),
        ("cast".to_string(), TokenType::Cast),
        //
        ("if".to_string(), TokenType::If),
        ("else".to_string(), TokenType::Else),
        ("while".to_string(), TokenType::While),
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
                        '!' => todo!("Read"),
                        _ => panic!("Unhandled character"),
                    }
                };
                tokens.push(Token::new(token_type, line_number));
            }
            '+' | '-' | '*' | '{' | '}' => {
                let token = match c {
                    '+' => TokenType::Add,
                    '-' => TokenType::Subtract,
                    '*' => TokenType::Multiply,
                    '{' => TokenType::LeftBrace,
                    '}' => TokenType::RightBrace,
                    _ => panic!("Unaccounted symbol"),
                };
                tokens.push(Token::new(token, line_number));
                chars.next();
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
                        '0'..='9' | '.' => {
                            s.push(c2);
                            chars.next();
                        }
                        _ => {
                            break;
                        }
                    }
                }

                tokens.push(Token::new(
                    TokenType::NumberValue(s.parse::<f64>().unwrap()),
                    line_number,
                ))
            }
            'A'..='Z' | 'a'..='z' => {
                let mut s = String::new();
                s.push(c);

                chars.next();
                while let Some(&c2) = chars.peek() {
                    if !c2.is_digit(10) && !c2.is_alphabetic() {
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

pub fn lex_string(input: &str) -> Vec<Token> {
    let result = scan_tokens(input);

    if let Ok(tokens) = result {
        return tokens;
    } else {
        panic!("Lexer failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_characters() {
        let input = String::from("+ - {} *\t/");
        let result = scan_tokens(&input);
        match result {
            Ok(r) => {
                let output = format!("{:?}", r);
                assert_eq!(
                    format!(
                        "{:?}",
                        vec![
                            Token::new(TokenType::Add, 1),
                            Token::new(TokenType::Subtract, 1),
                            Token::new(TokenType::LeftBrace, 1),
                            Token::new(TokenType::RightBrace, 1),
                            Token::new(TokenType::Multiply, 1),
                            Token::new(TokenType::Divide, 1)
                        ]
                    ),
                    output
                )
            }
            Err(_) => println!("Error"),
        }
    }

    #[test]
    fn parse_keywords() {
        let input = String::from("string int cast print true false \"Hello, World!\"");
        let result = scan_tokens(&input);
        match result {
            Ok(r) => {
                let output = format!("{:?}", r);
                assert_eq!(
                    format!(
                        "{:?}",
                        vec![
                            Token::new(TokenType::Type(Types::String), 1),
                            Token::new(TokenType::Type(Types::Int), 1),
                            Token::new(TokenType::Cast, 1),
                            Token::new(TokenType::Print, 1),
                            Token::new(TokenType::BoolValue(true), 1),
                            Token::new(TokenType::BoolValue(false), 1),
                            Token::new(TokenType::StringValue("Hello, World!".to_string()), 1)
                        ]
                    ),
                    output
                )
            }

            Err(_) => println!("Error"),
        }
    }

    #[test]
    fn parse_numbers() {
        let input = String::from("0 10 1234 0.123 1000.09123");
        let result = scan_tokens(&input);
        match result {
            Ok(r) => {
                let output = format!("{:?}", r);
                assert_eq!(
                    format!(
                        "{:?}",
                        vec![
                            Token::new(TokenType::NumberValue(0.0), 1),
                            Token::new(TokenType::NumberValue(10.0), 1),
                            Token::new(TokenType::NumberValue(1234.0), 1),
                            Token::new(TokenType::NumberValue(0.123), 1),
                            Token::new(TokenType::NumberValue(1000.09123), 1),
                        ]
                    ),
                    output
                )
            }

            Err(_) => println!("Error"),
        }
    }

    #[test]
    fn parse_escape_lines() {
        let input = String::from(r#" "\n \t \" " "#);
        let result = scan_tokens(&input);
        match result {
            Ok(r) => {
                let output = format!("{:?}", r);
                assert_eq!(
                    format!(
                        "{:?}",
                        vec![Token::new(
                            TokenType::StringValue("\n \t \" ".to_string()),
                            1
                        ),]
                    ),
                    output
                )
            }

            Err(_) => println!("Error"),
        }
    }

    #[test]
    fn parse_comments() {
        let input = String::from("int // Hello World\n int");
        let result = scan_tokens(&input);
        match result {
            Ok(r) => {
                let output = format!("{:?}", r);
                assert_eq!(
                    format!(
                        "{:?}",
                        vec![
                            Token::new(TokenType::Type(Types::Int), 1),
                            Token::new(TokenType::Type(Types::Int), 2)
                        ]
                    ),
                    output
                )
            }

            Err(_) => println!("Error"),
        }
    }

    #[test]
    fn parse_arithmetic() {
        let input = String::from(
            r#"
            1 2 +
            3 4 +
            *
            string cast print
            "#,
        );
        let result = scan_tokens(&input);
        match result {
            Ok(r) => {
                let output = format!("{:?}", r);
                assert_eq!(
                    format!(
                        "{:?}",
                        vec![
                            Token::new(TokenType::NumberValue(1.0), 2),
                            Token::new(TokenType::NumberValue(2.0), 2),
                            Token::new(TokenType::Add, 2),
                            Token::new(TokenType::NumberValue(3.0), 3),
                            Token::new(TokenType::NumberValue(4.0), 3),
                            Token::new(TokenType::Add, 3),
                            Token::new(TokenType::Multiply, 4),
                            Token::new(TokenType::Type(Types::String), 5),
                            Token::new(TokenType::Cast, 5),
                            Token::new(TokenType::Print, 5),
                        ]
                    ),
                    output
                )
            }

            Err(_) => println!("Error"),
        }
    }

    #[test]
    fn parse_boolean() {
        let input = String::from(r#"> < == != <= >="#);
        let result = scan_tokens(&input);
        match result {
            Ok(r) => {
                let output = format!("{:?}", r);
                assert_eq!(
                    format!(
                        "{:?}",
                        vec![
                            Token::new(TokenType::Greater, 1),
                            Token::new(TokenType::Less, 1),
                            Token::new(TokenType::Equal, 1),
                            Token::new(TokenType::NotEqual, 1),
                            Token::new(TokenType::LessEqual, 1),
                            Token::new(TokenType::GreaterEqual, 1),
                        ]
                    ),
                    output
                )
            }

            Err(_) => println!("Error"),
        }
    }

    #[test]
    fn parse_stack_operations() {
        let input = String::from(r#"rot dup drop over swap print cast"#);
        let result = scan_tokens(&input);
        match result {
            Ok(r) => {
                let output = format!("{:?}", r);
                assert_eq!(
                    format!(
                        "{:?}",
                        vec![
                            Token::new(TokenType::Rotate, 1),
                            Token::new(TokenType::Duplicate, 1),
                            Token::new(TokenType::Drop, 1),
                            Token::new(TokenType::Over, 1),
                            Token::new(TokenType::Swap, 1),
                            Token::new(TokenType::Print, 1),
                            Token::new(TokenType::Cast, 1),
                        ]
                    ),
                    output
                )
            }

            Err(_) => println!("Error"),
        }
    }

    #[test]
    fn parse_while_loop() {
        let input = String::from(r#"0 while dup 1 > {1 +}"#);
        let result = scan_tokens(&input);
        match result {
            Ok(r) => {
                let output = format!("{:?}", r);
                assert_eq!(
                    format!(
                        "{:?}",
                        vec![
                            Token::new(TokenType::NumberValue(0.0), 1),
                            Token::new(TokenType::While, 1),
                            Token::new(TokenType::Duplicate, 1),
                            Token::new(TokenType::NumberValue(1.0), 1),
                            Token::new(TokenType::Greater, 1),
                            Token::new(TokenType::LeftBrace, 1),
                            Token::new(TokenType::NumberValue(1.0), 1),
                            Token::new(TokenType::Add, 1),
                            Token::new(TokenType::RightBrace, 1),
                        ]
                    ),
                    output
                )
            }

            Err(_) => println!("Error"),
        }
    }

    #[test]
    fn parse_if() {
        let input = String::from(r#"0 if 1 > { "Less\n" print } else { "Greater\n" print }"#);
        let result = scan_tokens(&input);
        match result {
            Ok(r) => {
                let output = format!("{:?}", r);
                assert_eq!(
                    format!(
                        "{:?}",
                        vec![
                            Token::new(TokenType::NumberValue(0.0), 1),
                            Token::new(TokenType::If, 1),
                            Token::new(TokenType::NumberValue(1.0), 1),
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
                        ]
                    ),
                    output
                )
            }

            Err(_) => println!("Error"),
        }
    }
}
