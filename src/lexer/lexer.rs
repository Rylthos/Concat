use crate::lexer::tokens::{self, PositionInfo};
use std::collections::HashMap;

use crate::config::config::Config;
use crate::error::lexer_error::LexerError;
use crate::input::read_file_path;

use std::collections::HashSet;
use std::path::{Component, PathBuf};

use std::env;
use std::iter::Peekable;

use tokens::{Token, TokenType};

pub struct Lexer {
    config: Config,
    main_file: PathBuf,

    pub(crate) processing_files: HashSet<String>,
    pub(crate) processed_files: HashSet<String>,
}

impl Lexer {
    pub fn init(config: Config, main_file: PathBuf) -> Lexer {
        Lexer {
            config,
            main_file: main_file,
            processed_files: HashSet::new(),
            processing_files: HashSet::new(),
        }
    }

    pub fn lex_input(&mut self) -> Result<Vec<Token>, LexerError> {
        self.main_file = match self.main_file.canonicalize() {
            Ok(f) => f,
            Err(_) => {
                return Err(LexerError::InvalidFile(
                    self.main_file.clone(),
                    self.get_filename(&self.main_file),
                ));
            }
        };

        let tokens = self.scan_file(&self.main_file.clone(), "".to_string())?;

        if self.config.lexer_print {
            println!("=== TOKENS ===");
            for token in tokens.iter() {
                println!("{}", token);
            }
            println!("=== TOKENS ===");
        }

        return Ok(tokens);
    }

    fn parse_digit<I>(&self, chars: &mut Peekable<I>, is_negative: bool) -> (TokenType, String)
    where
        I: Iterator<Item = (usize, char)>,
    {
        let mut s = String::new();
        if is_negative {
            s.push('-');
        }

        let mut radix = 10;

        let (_, c) = chars.peek().unwrap().clone();
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

        if let Some(&(_, c2)) = chars.peek() {
            match c2 {
                'o' => {
                    radix = 8;
                    chars.next();
                }
                'x' => {
                    radix = 16;
                    chars.next();
                }
                'b' => {
                    radix = 2;
                    chars.next();
                }
                _ => (),
            }
        }

        let value = i32::from_str_radix(&s, radix).unwrap();
        (TokenType::I32Value(value), s)
    }

    fn calculate_file(
        &self,
        base_path: &PathBuf,
        file_name: &String,
    ) -> Result<PathBuf, LexerError> {
        let mut calculated_path = base_path.clone();
        calculated_path.pop();
        calculated_path.push(file_name);

        if *base_path == self.main_file && file_name == "" {
            return Ok(self.main_file.clone());
        }

        if file_name.starts_with("std:") {
            match env::var("CONCAT_STD_DIR") {
                Ok(o) => {
                    let mut path = PathBuf::from(o);
                    path.push(file_name.strip_prefix("std:").unwrap());
                    if path.is_file() {
                        return Ok(path.canonicalize().unwrap());
                    } else {
                        return Err(LexerError::InvalidStdFile(file_name.to_string()));
                    }
                }
                Err(_) => return Err(LexerError::InvalidStdFile(file_name.to_string())),
            }
        }

        if calculated_path.is_file() {
            Ok(calculated_path.canonicalize().unwrap())
        } else {
            Err(LexerError::InvalidFile(
                base_path.clone(),
                file_name.to_string(),
            ))
        }
    }

    pub(crate) fn scan_file(
        &mut self,
        base_path: &PathBuf,
        file_name: String,
    ) -> Result<Vec<Token>, LexerError> {
        let file = self.calculate_file(base_path, &file_name)?;

        let input = read_file_path(&file);
        let mut tokens = self.scan_string(&file, &input)?;

        self.process_includes(&file, &mut tokens)?;

        Ok(tokens)
    }

    pub fn scan_string(
        &mut self,
        file: &std::path::PathBuf,
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
            ("include".to_string(), TokenType::Include("".to_string())),
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
            ("syscall".to_string(), TokenType::Syscall),
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
                    if let Some(&(_, c2)) = chars.peek()
                        && c2 == '&'
                    {
                        tokens.push(Token::new(
                            TokenType::And,
                            line_number,
                            column_number,
                            &self.get_filename(&file),
                            "&&",
                        ));
                        chars.next();
                    } else {
                        tokens.push(Token::new(
                            TokenType::BitwiseAnd,
                            line_number,
                            column_number,
                            &self.get_filename(&file),
                            "&",
                        ));
                    }
                }
                '|' => {
                    chars.next();
                    if let Some(&(_, c2)) = chars.peek()
                        && c2 == '|'
                    {
                        tokens.push(Token::new(
                            TokenType::Or,
                            line_number,
                            column_number,
                            &self.get_filename(&file),
                            "||",
                        ));
                        chars.next();
                    } else {
                        tokens.push(Token::new(
                            TokenType::BitwiseOr,
                            line_number,
                            column_number,
                            &self.get_filename(&file),
                            "|",
                        ));
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
                        && c2.is_digit(10)
                    {
                        let (token, s) = self.parse_digit(&mut chars, true);
                        tokens.push(Token::new(
                            token,
                            line_number,
                            column_number,
                            &self.get_filename(&file),
                            &s,
                        ));
                    } else if let Some(&(_, c2)) = chars.peek()
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
                    let (token_type, s) = self.parse_digit(&mut chars, false);
                    tokens.push(Token::new(
                        token_type,
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
                            TokenType::Include(_) => tokens.push(Token::new(
                                TokenType::Include(tokens.last().unwrap().string.clone()),
                                line_number,
                                column_number,
                                &self.get_filename(&file),
                                &s,
                            )),
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
                                    && let Some((_, char4)) = characters.next()
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

    pub(crate) fn get_filename(&self, file: &PathBuf) -> String {
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
