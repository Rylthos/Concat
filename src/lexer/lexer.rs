use crate::lexer::tokens::{self, PositionInfo};
use std::collections::HashMap;

use crate::config::config::Config;
use crate::error::lexer_error::LexerError;
use crate::input::read_file_path;

use std::collections::HashSet;
use std::path::{Component, PathBuf};

use tokens::{Token, TokenType};

pub struct Lexer {
    config: Config,
    main_file: PathBuf,

    processed_files: HashSet<String>,

    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn init(config: Config, main_file: PathBuf) -> Lexer {
        Lexer {
            config,
            main_file: main_file,
            processed_files: HashSet::new(),
            tokens: Vec::new(),
        }
    }

    pub fn lex_input(&mut self) -> Result<(), LexerError> {
        self.main_file = match self.main_file.canonicalize() {
            Ok(f) => f,
            Err(_) => {
                return Err(LexerError::InvalidFile(self.get_filename(&self.main_file)));
            }
        };

        self.tokens = match self.scan_file(self.main_file.clone()) {
            Ok(t) => t,
            Err(err) => return Err(err),
        };

        if self.config.lexer_print {
            println!("=== TOKENS ===");
            for token in self.tokens.iter() {
                println!("{}", token);
            }
            println!("=== TOKENS ===");
        }

        return Ok(());
    }

    pub fn scan_file(&mut self, file: std::path::PathBuf) -> Result<Vec<Token>, LexerError> {
        if !file.is_file() {
            return Err(LexerError::InvalidFile(self.get_filename(&file)));
        }

        let filename = self.get_filename(&file);

        if self.processed_files.contains(&filename) {
            return Err(LexerError::CircularInclude(filename));
        }

        self.processed_files.insert(self.get_filename(&file));

        let input = read_file_path(&file);
        self.scan_string(file, &input)
    }

    pub fn scan_string(
        &mut self,
        file: std::path::PathBuf,
        input: &str,
    ) -> Result<Vec<Token>, LexerError> {
        let raw_chars = input.chars();

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
            ("include".to_string(), TokenType::Include),
            //
            ("true".to_string(), TokenType::BoolValue(true)),
            ("false".to_string(), TokenType::BoolValue(false)),
            //
            ("bool".to_string(), TokenType::Bool),
            ("i32".to_string(), TokenType::I32),
            ("void".to_string(), TokenType::Void),
            ("char".to_string(), TokenType::Char),
            ("const".to_string(), TokenType::Const),
            //
            ("rot3".to_string(), TokenType::Rotate3),
            ("dup".to_string(), TokenType::Duplicate),
            ("drop".to_string(), TokenType::Drop),
            ("over".to_string(), TokenType::Over),
            ("swap".to_string(), TokenType::Swap),
            ("print".to_string(), TokenType::Print),
            //
            ("union".to_string(), TokenType::Union),
            ("nth".to_string(), TokenType::Nth),
            //
            ("if".to_string(), TokenType::If),
            ("else".to_string(), TokenType::Else),
            ("while".to_string(), TokenType::While),
            //
            ("input".to_string(), TokenType::Input),
            //
            ("func".to_string(), TokenType::Func),
            //
            ("record".to_string(), TokenType::Record),
            //
            ("assign".to_string(), TokenType::Assignment),
            //
            ("mem".to_string(), TokenType::Mem),
            //
            ("define".to_string(), TokenType::Define),
            //
            ("__PRINT_STACK__".to_string(), TokenType::DebugPrintStack),
            ("__PRINT_HEAP__".to_string(), TokenType::DebugHeapStack),
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
                        &self.get_filename(&file),
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
                                &self.get_filename(&file),
                                "&&",
                            ));
                            chars.next();
                        } else {
                            return Err(LexerError::InvalidToken(
                                PositionInfo {
                                    line: line_number,
                                    column: column_number,
                                    file: self.get_filename(&file),
                                },
                                format!("&{:?}", c2),
                            ));
                        }
                    } else {
                        return Err(LexerError::ExpectedCharacter(PositionInfo {
                            line: line_number,
                            column: column_number,
                            file: self.get_filename(&file),
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
                                &self.get_filename(&file),
                                "||",
                            ));
                            chars.next();
                        } else {
                            return Err(LexerError::InvalidToken(
                                PositionInfo {
                                    line: line_number,
                                    column: column_number,
                                    file: self.get_filename(&file),
                                },
                                format!("|{:?}", c2),
                            ));
                        }
                    } else {
                        return Err(LexerError::ExpectedCharacter(PositionInfo {
                            line: line_number,
                            column: column_number,
                            file: self.get_filename(&file),
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
                            '=' => TokenType::Assign,
                            '!' => TokenType::Exclamation,
                            _ => unreachable!("Unhandled case"),
                        }
                    };
                    tokens.push(Token::new(
                        token_type,
                        line_number,
                        column_number,
                        &self.get_filename(&file),
                        &lexed_string,
                    ));
                }
                '+' | '*' | '%' | '{' | '}' | '@' | '[' | ']' => {
                    let token = match c {
                        '+' => TokenType::Add,
                        '*' => TokenType::Asterisk,
                        '%' => TokenType::Modulo,
                        '{' => TokenType::LeftBrace,
                        '}' => TokenType::RightBrace,
                        '[' => TokenType::LeftSqBracket,
                        ']' => TokenType::RightSqBracket,
                        '@' => TokenType::Read,
                        _ => unreachable!("Unhandled case"),
                    };
                    tokens.push(Token::new(
                        token,
                        line_number,
                        column_number,
                        &self.get_filename(&file),
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
                            &self.get_filename(&file),
                            "->",
                        ));
                        chars.next();
                    } else {
                        tokens.push(Token::new(
                            TokenType::Subtract,
                            line_number,
                            column_number,
                            &self.get_filename(&file),
                            "-",
                        ));
                    }
                }
                '\'' => {
                    let c: char;
                    chars.next();

                    c = Self::read_char(
                        PositionInfo::new(line_number, column_number, &self.get_filename(&file)),
                        &mut chars,
                    )?;

                    if let Some((_, c2)) = chars.next() {
                        if c2 == '\'' {
                        } else {
                            return Err(LexerError::ExpectedCharacterGot(
                                PositionInfo::new(
                                    line_number,
                                    column_number,
                                    &self.get_filename(&file),
                                ),
                                '\'',
                                c2,
                            ));
                        }
                    }

                    tokens.push(Token::new(
                        TokenType::CharValue(c),
                        line_number,
                        column_number,
                        &self.get_filename(&file),
                        &format!("'{}'", c),
                    ));
                }
                '"' => {
                    let mut s = String::new();

                    chars.next();
                    while let Some((_, c2)) = chars.peek() {
                        if *c2 == '"' {
                            chars.next();
                            break;
                        } else {
                            let c = Self::read_char(
                                PositionInfo::new(
                                    line_number,
                                    column_number,
                                    &self.get_filename(&file),
                                ),
                                &mut chars,
                            )?;
                            s.push(c);
                        }
                    }

                    tokens.push(Token::new(
                        TokenType::StringValue(s.clone()),
                        line_number,
                        column_number,
                        &self.get_filename(&file),
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
                        TokenType::I32Value(s.parse::<i32>().unwrap()),
                        line_number,
                        column_number,
                        &self.get_filename(&file),
                        &s,
                    ))
                }
                '_' | 'A'..='Z' | 'a'..='z' | '.' => {
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
                        Some(t) => match t {
                            TokenType::Include => tokens.append(&mut self.read_file(
                                &file,
                                &PositionInfo::new(
                                    line_number,
                                    column_number,
                                    &self.get_filename(&file),
                                ),
                                tokens.last(),
                            )?),
                            _ => tokens.push(Token::new(
                                t.clone(),
                                line_number,
                                column_number,
                                &self.get_filename(&file),
                                &s,
                            )),
                        },
                        None => {
                            if c == '.' {
                                tokens.push(Token::new(
                                    TokenType::RecordIdentifier(s[1..].to_string().clone()),
                                    line_number,
                                    column_number,
                                    &self.get_filename(&file),
                                    &s,
                                ));
                            } else {
                                tokens.push(Token::new(
                                    TokenType::Identifier(s.clone()),
                                    line_number,
                                    column_number,
                                    &self.get_filename(&file),
                                    &s,
                                ));
                            }
                        }
                    }
                }
                _ => {
                    return Err(LexerError::InvalidCharacter(
                        PositionInfo {
                            line: line_number,
                            column: column_number,
                            file: self.get_filename(&file),
                        },
                        c,
                    ));
                }
            }
        }

        Ok(tokens)
    }

    fn read_file(
        &mut self,
        current_file: &PathBuf,
        pos: &PositionInfo,
        previous_token: Option<&Token>,
    ) -> Result<Vec<Token>, LexerError> {
        let token = match previous_token {
            Some(s) => s,
            None => return Err(LexerError::ExpectedFilePath(pos.clone())),
        };

        let filepath = match &token.token_type {
            TokenType::StringValue(s) => s,
            _ => return Err(LexerError::InvalidInclude(pos.clone(), token.clone())),
        };

        let mut path = current_file.clone();
        path.pop();
        path.push(filepath);

        path = match path.canonicalize() {
            Ok(f) => f,
            Err(_) => {
                return Err(LexerError::InvalidFile(filepath.to_string()));
            }
        };

        Ok(self.scan_file(path)?)
    }

    fn read_char<I>(
        base_position: PositionInfo,
        characters: &mut std::iter::Peekable<I>,
    ) -> Result<char, LexerError>
    where
        I: Iterator<Item = (usize, char)>,
    {
        if let Some((col1, char1)) = characters.next() {
            match char1 {
                '\\' => {
                    if let Some((col2, char2)) = characters.next() {
                        match char2 {
                            '0' => return Ok('\0'),
                            '\\' => return Ok('\\'),
                            '\"' => return Ok('\"'),
                            '\'' => return Ok('\''),
                            'n' => return Ok('\n'),
                            't' => return Ok('\t'),
                            'x' => {
                                if let Some((col3, char3)) = characters.next()
                                    && let Some((col4, char4)) = characters.next()
                                {
                                    if char3.is_digit(16) && char4.is_digit(16) {
                                        let value = char3.to_digit(16).unwrap() * 16
                                            + char4.to_digit(16).unwrap();
                                        return Ok(char::from_u32(value).unwrap());
                                    } else {
                                        return Err(LexerError::InvalidCharacter(
                                            PositionInfo::new(
                                                base_position.line,
                                                col3,
                                                &base_position.file,
                                            ),
                                            char2,
                                        ));
                                    }
                                } else {
                                    return Err(LexerError::ExpectedCharacter(PositionInfo::new(
                                        base_position.line,
                                        col2,
                                        &base_position.file,
                                    )));
                                }
                            }
                            _ => {
                                return Err(LexerError::InvalidCharacter(
                                    PositionInfo::new(
                                        base_position.line,
                                        col2,
                                        &base_position.file,
                                    ),
                                    char2,
                                ));
                            }
                        }
                    } else {
                        return Err(LexerError::ExpectedCharacter(PositionInfo::new(
                            base_position.line,
                            col1,
                            &base_position.file,
                        )));
                    }
                }
                _ => {
                    return Ok(char1.clone());
                }
            }
        } else {
            return Err(LexerError::ExpectedCharacter(base_position));
        }
    }

    fn get_filename(&self, file: &PathBuf) -> String {
        if file.clone().into_os_string() == self.main_file.clone().into_os_string() {
            return match file.file_name() {
                Some(s) => s.to_str().expect("").to_string(),
                None => "".to_string(),
            };
        }

        let base: Vec<_> = self.main_file.as_path().components().collect();
        let target: Vec<_> = file.as_path().components().collect();

        let common = base
            .iter()
            .zip(target.iter())
            .take_while(|(a, b)| a == b)
            .count();

        let mut result = PathBuf::new();
        for _ in common..(base.len() - 1) {
            result.push("..");
        }

        for component in &target[common..] {
            match component {
                Component::Normal(c) => result.push(c),
                Component::CurDir | Component::RootDir | Component::Prefix(_) => {}
                Component::ParentDir => result.push(".."),
            }
        }

        return result.to_str().expect("").to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input(input: &str, expected_output: &Vec<Token>) {
        let mut lexer = Lexer::init(Config::blank(), PathBuf::new());
        let result = lexer.scan_string(PathBuf::new(), input);
        match result {
            Ok(t) => assert_eq!(format!("{:?}", expected_output), format!("{:?}", t)),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[test]
    fn lex_single_characters() {
        let input = "+ - {} *\t/ []";
        let output = vec![
            Token::new(TokenType::Add, 1, 1, "", "+"),
            Token::new(TokenType::Subtract, 1, 3, "", "-"),
            Token::new(TokenType::LeftBrace, 1, 5, "", "{"),
            Token::new(TokenType::RightBrace, 1, 6, "", "}"),
            Token::new(TokenType::Asterisk, 1, 8, "", "*"),
            Token::new(TokenType::Divide, 1, 10, "", "/"),
            Token::new(TokenType::LeftSqBracket, 1, 12, "", "["),
            Token::new(TokenType::RightSqBracket, 1, 13, "", "]"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_keywords() {
        let input = "i32 void bool print true false char '\\0' \"Hello, World!\"";
        let output = vec![
            Token::new(TokenType::I32, 1, 1, "", "i32"),
            Token::new(TokenType::Void, 1, 5, "", "void"),
            Token::new(TokenType::Bool, 1, 10, "", "bool"),
            Token::new(TokenType::Print, 1, 15, "", "print"),
            Token::new(TokenType::BoolValue(true), 1, 21, "", "true"),
            Token::new(TokenType::BoolValue(false), 1, 26, "", "false"),
            Token::new(TokenType::Char, 1, 32, "", "char"),
            Token::new(TokenType::CharValue('\0'), 1, 37, "", "'\0'"),
            Token::new(
                TokenType::StringValue("Hello, World!".to_string()),
                1,
                42,
                "",
                "\"Hello, World!\"",
            ),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_numbers() {
        let input = "0 10 1234";
        let output = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", "0"),
            Token::new(TokenType::I32Value(10), 1, 3, "", "10"),
            Token::new(TokenType::I32Value(1234), 1, 6, "", "1234"),
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
            "",
            "\"\\n \\t \\\" \"",
        )];
        test_input(input, &output);
    }

    #[test]
    fn lex_comments() {
        let input = "i32 // Hello World\n i32";
        let output = vec![
            Token::new(TokenType::I32, 1, 1, "", "i32"),
            Token::new(TokenType::I32, 2, 2, "", "i32"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_arithmetic() {
        let input = "\n1 2 +\n3 4 +\n*\nprint";
        let output = vec![
            Token::new(TokenType::I32Value(1), 2, 1, "", "1"),
            Token::new(TokenType::I32Value(2), 2, 3, "", "2"),
            Token::new(TokenType::Add, 2, 5, "", "+"),
            Token::new(TokenType::I32Value(3), 3, 1, "", "3"),
            Token::new(TokenType::I32Value(4), 3, 3, "", "4"),
            Token::new(TokenType::Add, 3, 5, "", "+"),
            Token::new(TokenType::Asterisk, 4, 1, "", "*"),
            Token::new(TokenType::Print, 5, 1, "", "print"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_boolean() {
        let input = r#"> < == != <= >="#;
        let output = vec![
            Token::new(TokenType::Greater, 1, 1, "", ">"),
            Token::new(TokenType::Less, 1, 3, "", "<"),
            Token::new(TokenType::Equal, 1, 5, "", "=="),
            Token::new(TokenType::NotEqual, 1, 8, "", "!="),
            Token::new(TokenType::LessEqual, 1, 11, "", "<="),
            Token::new(TokenType::GreaterEqual, 1, 14, "", ">="),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_stack_operations() {
        let input = r#"rot3 dup drop over swap print"#;
        let output = vec![
            Token::new(TokenType::Rotate3, 1, 1, "", "rot3"),
            Token::new(TokenType::Duplicate, 1, 6, "", "dup"),
            Token::new(TokenType::Drop, 1, 10, "", "drop"),
            Token::new(TokenType::Over, 1, 15, "", "over"),
            Token::new(TokenType::Swap, 1, 20, "", "swap"),
            Token::new(TokenType::Print, 1, 25, "", "print"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_while_loop() {
        let input = r#"0 while dup 1 > {1 +}"#;
        let output = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", "0"),
            Token::new(TokenType::While, 1, 3, "", "while"),
            Token::new(TokenType::Duplicate, 1, 9, "", "dup"),
            Token::new(TokenType::I32Value(1), 1, 13, "", "1"),
            Token::new(TokenType::Greater, 1, 15, "", ">"),
            Token::new(TokenType::LeftBrace, 1, 17, "", "{"),
            Token::new(TokenType::I32Value(1), 1, 18, "", "1"),
            Token::new(TokenType::Add, 1, 20, "", "+"),
            Token::new(TokenType::RightBrace, 1, 21, "", "}"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_if() {
        let input = r#"0 if 1 > { "Less\n" print } else { "Greater\n" print }"#;
        let output = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", "0"),
            Token::new(TokenType::If, 1, 3, "", "if"),
            Token::new(TokenType::I32Value(1), 1, 6, "", "1"),
            Token::new(TokenType::Greater, 1, 8, "", ">"),
            Token::new(TokenType::LeftBrace, 1, 10, "", "{"),
            Token::new(
                TokenType::StringValue("Less\n".to_string()),
                1,
                12,
                "",
                "\"Less\\n\"",
            ),
            Token::new(TokenType::Print, 1, 21, "", "print"),
            Token::new(TokenType::RightBrace, 1, 27, "", "}"),
            Token::new(TokenType::Else, 1, 29, "", "else"),
            Token::new(TokenType::LeftBrace, 1, 34, "", "{"),
            Token::new(
                TokenType::StringValue("Greater\n".to_string()),
                1,
                36,
                "",
                "\"Greater\\n\"",
            ),
            Token::new(TokenType::Print, 1, 48, "", "print"),
            Token::new(TokenType::RightBrace, 1, 54, "", "}"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_func() {
        let input = r#"func test i32 i32 -> i32 { + }"#;
        let output = vec![
            Token::new(TokenType::Func, 1, 1, "", "func"),
            Token::new(TokenType::Identifier("test".to_string()), 1, 6, "", "test"),
            Token::new(TokenType::I32, 1, 11, "", "i32"),
            Token::new(TokenType::I32, 1, 15, "", "i32"),
            Token::new(TokenType::Arrow, 1, 19, "", "->"),
            Token::new(TokenType::I32, 1, 22, "", "i32"),
            Token::new(TokenType::LeftBrace, 1, 26, "", "{"),
            Token::new(TokenType::Add, 1, 28, "", "+"),
            Token::new(TokenType::RightBrace, 1, 30, "", "}"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_variables() {
        let input = r#"0 assign x { x @ print x 1 = }"#;
        let output = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "", "0"),
            Token::new(TokenType::Assignment, 1, 3, "", "assign"),
            Token::new(TokenType::Identifier("x".to_string()), 1, 10, "", "x"),
            Token::new(TokenType::LeftBrace, 1, 12, "", "{"),
            Token::new(TokenType::Identifier("x".to_string()), 1, 14, "", "x"),
            Token::new(TokenType::Read, 1, 16, "", "@"),
            Token::new(TokenType::Print, 1, 18, "", "print"),
            Token::new(TokenType::Identifier("x".to_string()), 1, 24, "", "x"),
            Token::new(TokenType::I32Value(1), 1, 26, "", "1"),
            Token::new(TokenType::Assign, 1, 28, "", "="),
            Token::new(TokenType::RightBrace, 1, 30, "", "}"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn lex_record() {
        let input = r#"record temp { i32 v1 i32 v2 } temp dup .v1 swap 1 .v2!"#;
        let output = vec![
            Token::new(TokenType::Record, 1, 1, "", "record"),
            Token::new(TokenType::Identifier("temp".to_string()), 1, 8, "", "temp"),
            Token::new(TokenType::LeftBrace, 1, 13, "", "{"),
            Token::new(TokenType::I32, 1, 15, "", "i32"),
            Token::new(TokenType::Identifier("v1".to_string()), 1, 19, "", "v1"),
            Token::new(TokenType::I32, 1, 22, "", "i32"),
            Token::new(TokenType::Identifier("v2".to_string()), 1, 26, "", "v2"),
            Token::new(TokenType::RightBrace, 1, 29, "", "}"),
            Token::new(TokenType::Identifier("temp".to_string()), 1, 31, "", "temp"),
            Token::new(TokenType::Duplicate, 1, 36, "", "dup"),
            Token::new(
                TokenType::RecordIdentifier("v1".to_string()),
                1,
                40,
                "",
                ".v1",
            ),
            Token::new(TokenType::Swap, 1, 44, "", "swap"),
            Token::new(TokenType::I32Value(1), 1, 49, "", "1"),
            Token::new(
                TokenType::RecordIdentifier("v2".to_string()),
                1,
                51,
                "",
                ".v2",
            ),
            Token::new(TokenType::Exclamation, 1, 54, "", "!"),
        ];
        test_input(input, &output);
    }

    #[test]
    fn filename() {
        let lexer = Lexer::init(Config::blank(), PathBuf::from("test/test/test.concat"));

        assert_eq!(
            lexer.get_filename(&PathBuf::from("test/test/test.concat")),
            "test.concat"
        );

        assert_eq!(
            lexer.get_filename(&PathBuf::from("test/test.concat")),
            "../test.concat"
        );

        assert_eq!(
            lexer.get_filename(&PathBuf::from("test/test2/test.concat")),
            "../test2/test.concat"
        );

        assert_eq!(
            lexer.get_filename(&PathBuf::from("test/test/test3/test.concat")),
            "test3/test.concat"
        );

        assert_eq!(
            lexer.get_filename(&PathBuf::from("test/test/test2/test.concat")),
            "test2/test.concat"
        );
    }
}
