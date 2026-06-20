use crate::error::types::{ErrorType, ParserError};
use crate::lexer::tokens::{Token, TokenType, Types};
use crate::parser::instructions::{Instruction, StackValue};
use crate::parser::parse_tree::ParseTree;
use crate::parser::typing::Typing;

use crate::config::config::Config;

use std::collections::HashMap;

pub struct Parser {
    config: Config,

    tokens: Vec<Token>,
    parse_tree: ParseTree,

    functions: HashMap<String, ParseTree>,

    pub instructions: Vec<Instruction>,
}

impl Parser {
    pub fn init(config: Config, tokens: Vec<Token>) -> Parser {
        Parser {
            config,
            tokens,
            parse_tree: ParseTree::None,
            functions: HashMap::new(),
            instructions: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), ErrorType> {
        let tokens = self.tokens.clone();
        let tree = match self.generate_parse_tree(tokens.iter().peekable()) {
            Ok(t) => t,
            Err(e) => return Err(ErrorType::Parser(e)),
        };

        self.parse_tree = tree;

        if self.config.tree_print {
            self.print_tree();
        }

        match Typing::type_check(&self.parse_tree, &self.functions) {
            Ok(_) => (),
            Err(e) => return Err(ErrorType::Parser(e)),
        };

        self.instructions = match self.parse_tree(self.parse_tree.clone()) {
            Ok(i) => i,
            Err(e) => return Err(ErrorType::Parser(e)),
        };
        self.instructions.push(Instruction::Halt);

        let mut function_instructions = match self.parse_functions() {
            Ok(i) => i,
            Err(e) => return Err(ErrorType::Parser(e)),
        };
        self.instructions.append(&mut function_instructions);

        self.evaluate_labels();

        if self.config.expr_print {
            self.print_instr();
        }

        Ok(())
    }

    fn print_tree(&self) {
        println!("==== TREE ====");
        println!("{}", self.parse_tree);

        for (_, func) in self.functions.iter() {
            println!("{}", func);
        }
        println!("==== TREE ====");
    }

    fn print_instr(&self) {
        println!("==== INSTR ====");
        for instr in self.instructions.iter() {
            println!("{:?}", instr);
        }
        println!("==== INSTR ====");
    }

    fn generate_parse_tree<'a>(
        &mut self,
        tokens: impl Iterator<Item = &'a Token>,
    ) -> Result<ParseTree, ParserError> {
        let mut region: Vec<ParseTree> = Vec::new();
        let mut peekable = tokens.peekable();

        while let Some(&t) = peekable.peek() {
            match t.token_type {
                TokenType::If => {
                    peekable.next();
                    let mut regions = Vec::new();
                    let mut else_region = (t.clone(), Box::new(ParseTree::Region(vec![])));

                    loop {
                        let conditional_tree =
                            self.generate_parse_tree(Self::get_condition(&mut peekable)?.iter())?;
                        let region_tree =
                            self.generate_parse_tree(Self::get_region(&mut peekable)?.iter())?;

                        regions.push((
                            t.clone(),
                            Box::new(conditional_tree),
                            Box::new(region_tree),
                        ));

                        if let Some(&t) = peekable.peek() {
                            match t.token_type {
                                TokenType::Else => {
                                    peekable.next();
                                    if let Some(&t2) = peekable.peek() {
                                        match t2.token_type {
                                            TokenType::If => {
                                                peekable.next();
                                                continue;
                                            }
                                            _ => (),
                                        }
                                    }

                                    else_region = (
                                        t.clone(),
                                        Box::new(self.generate_parse_tree(
                                            Self::get_region(&mut peekable)?.iter(),
                                        )?),
                                    );

                                    break;
                                }
                                _ => break,
                            }
                        } else {
                            break;
                        }
                    }

                    region.push(ParseTree::If(regions, else_region));
                }
                TokenType::While => {
                    peekable.next();
                    let conditional_tree =
                        self.generate_parse_tree(Self::get_condition(&mut peekable)?.iter())?;
                    let region_tree =
                        self.generate_parse_tree(Self::get_region(&mut peekable)?.iter())?;

                    region.push(ParseTree::While(
                        t.clone(),
                        Box::new(conditional_tree),
                        Box::new(region_tree),
                    ))
                }
                TokenType::Func => {
                    peekable.next();
                    let function_name = if let Some(t) = peekable.next() {
                        match &t.token_type {
                            TokenType::Identifier(s) => s.clone(),
                            _ => {
                                return Err(ParserError::InvalidFunctionDef(
                                    t.position_info.clone(),
                                    t.token_type.clone(),
                                ));
                            }
                        }
                    } else {
                        return Err(ParserError::ExpectedToken(
                            t.position_info.clone(),
                            TokenType::Identifier("".to_string()),
                        ));
                    };

                    let input_types = Self::get_types(&mut peekable)?
                        .iter()
                        .filter(|i| match i {
                            Types::Void => false,
                            _ => true,
                        })
                        .cloned()
                        .collect();

                    if let Some(t) = peekable.next() {
                        match &t.token_type {
                            TokenType::Arrow => (),
                            _ => {
                                return Err(ParserError::ExpectedToken(
                                    t.position_info.clone(),
                                    TokenType::Arrow,
                                ));
                            }
                        }
                    } else {
                        return Err(ParserError::ExpectedToken(
                            t.position_info.clone(),
                            TokenType::Arrow,
                        ));
                    };

                    let output_types = Self::get_types(&mut peekable)?
                        .iter()
                        .filter(|i| match i {
                            Types::Void => false,
                            _ => true,
                        })
                        .cloned()
                        .collect();

                    let region_tree =
                        self.generate_parse_tree(Self::get_region(&mut peekable)?.iter())?;

                    self.functions.insert(
                        function_name.clone(),
                        ParseTree::FuncDecl(
                            t.clone(),
                            function_name,
                            input_types,
                            output_types,
                            Box::new(region_tree),
                        ),
                    );
                }
                _ => {
                    peekable.next();
                    region.push(ParseTree::Element(t.clone().clone()))
                }
            }
        }

        return Ok(ParseTree::Region(region));
    }

    fn parse_element(&self, token: &Token) -> Option<Instruction> {
        let instr = match token.token_type.clone() {
            TokenType::LeftBrace
            | TokenType::RightBrace
            | TokenType::If
            | TokenType::Else
            | TokenType::While
            | TokenType::Arrow
            | TokenType::Func
            | TokenType::Declare
            | TokenType::Assignment
            | TokenType::Read => {
                unreachable!("Unreachable: {:?}", token);
            }
            TokenType::StringValue(s) => Instruction::Push(StackValue::String(s.to_string())),
            TokenType::I32(n) => Instruction::Push(StackValue::I32(n)),
            TokenType::BoolValue(b) => Instruction::Push(StackValue::Bool(b)),
            TokenType::Type(t) => Instruction::Push(StackValue::Type(t.clone())),
            //
            TokenType::Add => Instruction::Add,
            TokenType::Subtract => Instruction::Subtract,
            TokenType::Divide => Instruction::Divide,
            TokenType::Multiply => Instruction::Multiply,
            TokenType::Modulo => Instruction::Modulo,
            //
            TokenType::Rotate3 => Instruction::Rotate3,
            TokenType::Duplicate => Instruction::Duplicate,
            TokenType::Drop => Instruction::Drop,
            TokenType::Over => Instruction::Over,
            TokenType::Swap => Instruction::Swap,
            TokenType::Print => Instruction::Print,
            //
            TokenType::Less => Instruction::Less,
            TokenType::LessEqual => Instruction::LessEqual,
            TokenType::Greater => Instruction::Greater,
            TokenType::GreaterEqual => Instruction::GreaterEqual,
            TokenType::Equal => Instruction::Equal,
            TokenType::NotEqual => Instruction::NotEqual,
            TokenType::And => Instruction::And,
            TokenType::Or => Instruction::Or,
            TokenType::Not => Instruction::Not,
            //
            TokenType::Identifier(_) => return None,
        };

        return Some(instr);
    }

    fn parse_tree<'a>(&self, tree: ParseTree) -> Result<Vec<Instruction>, ParserError> {
        let mut parsed_expression: Vec<Instruction> = Vec::new();

        match tree {
            ParseTree::None => return Err(ParserError::InvalidParseTree()),
            ParseTree::Element(e) => {
                let expr = self.parse_element(&e);
                if let Some(e) = expr {
                    parsed_expression.push(e)
                } else {
                    match e.token_type {
                        TokenType::Identifier(iden) => {
                            if self.functions.contains_key(&iden) {
                                if let ParseTree::FuncDecl(_, _, input_types, _, _) =
                                    self.functions.get(&iden).unwrap()
                                {
                                    parsed_expression.push(Instruction::Push(StackValue::I32(
                                        input_types.len() as i32,
                                    )));
                                    parsed_expression.push(Instruction::LabelRef(
                                        iden,
                                        Box::new(Instruction::Call(0)),
                                    ));
                                } else {
                                    unreachable!(
                                        "Valid function name should be a FuncDecl in ParseTree"
                                    );
                                }
                            } else {
                                return Err(ParserError::UnknownIdentifier(
                                    e.position_info,
                                    iden.to_string(),
                                ));
                            }
                        }
                        _ => {
                            return Err(ParserError::ExpectedTokenGot(
                                e.position_info,
                                TokenType::Identifier("".to_string()),
                                e.token_type.clone(),
                            ));
                        }
                    }
                }
            }
            ParseTree::Region(r) => parsed_expression.append(
                &mut r
                    .iter()
                    .map(|m| match self.parse_tree(m.clone()) {
                        Ok(r) => r,
                        Err(_) => panic!(),
                    })
                    .flatten()
                    .collect::<Vec<Instruction>>(),
            ),
            ParseTree::If(if_branches, (_, else_branch)) => {
                let if_branches_result = if_branches
                    .iter()
                    .map(|(_, c, m)| {
                        let c1 = self.parse_tree(*c.clone());
                        let c2 = self.parse_tree(*m.clone());
                        (c1, c2)
                    })
                    .collect::<Vec<(
                        Result<Vec<Instruction>, ParserError>,
                        Result<Vec<Instruction>, ParserError>,
                    )>>();

                let mut if_branches = Vec::new();
                for (c, r) in if_branches_result {
                    let c = c?;
                    let r = r?;
                    if_branches.push((c, r));
                }

                let mut else_branch = self.parse_tree(*else_branch)?;

                let total_length = if_branches
                    .iter()
                    .fold(0, |s, (c, r)| s + c.len() + r.len())
                    + (if_branches.len() - 1) // Jumps
                    + else_branch.len();

                let total_conditional_branches = if_branches.len();

                let mut length_seen = 0;
                let mut branches_seen = 0;
                for (mut c, mut r) in if_branches {
                    length_seen += c.len() + r.len();
                    let jump_length = total_length - length_seen;

                    parsed_expression.append(&mut c);
                    parsed_expression.push(Instruction::CondJump(
                        1,
                        r.len() + 1 + if jump_length != 0 { 1 } else { 0 },
                    ));
                    parsed_expression.append(&mut r);

                    if jump_length != 0 {
                        length_seen += 1;
                        parsed_expression.push(Instruction::Jump(
                            (jump_length + (total_conditional_branches - branches_seen)) as isize,
                        ));
                    }

                    branches_seen += 1;
                }

                parsed_expression.append(&mut else_branch);
            }
            ParseTree::While(_, c, r) => {
                let mut condition_tree = self.parse_tree(*c)?;
                let mut region_tree = self.parse_tree(*r)?;

                let total_length = condition_tree.len() + region_tree.len();
                parsed_expression.append(&mut condition_tree);
                parsed_expression.push(Instruction::CondJump(1, region_tree.len() + 2));
                parsed_expression.append(&mut region_tree);
                parsed_expression.push(Instruction::Jump(-(total_length as isize) - 1));
            }
            ParseTree::FuncDecl(_, name, _, output_types, region) => {
                let mut region = self.parse_tree(*region)?;

                region.push(Instruction::Push(
                    StackValue::I32(output_types.len() as i32),
                ));
                region.push(Instruction::Ret);
                let initial_token = region.get(0).unwrap();
                *region.get_mut(0).unwrap() =
                    Instruction::Label(name, Box::new(initial_token.clone()));

                parsed_expression.append(&mut region);
            }
        }

        return Ok(parsed_expression);
    }

    fn evaluate_labels(&mut self) {
        let labels: HashMap<String, usize> = self
            .instructions
            .iter()
            .zip(0..)
            .filter(|(instr, _)| match instr {
                Instruction::Label(_, _) => true,
                _ => false,
            })
            .map(|(instr, i)| match instr {
                Instruction::Label(name, _) => (name.to_string(), i),
                _ => unreachable!(),
            })
            .collect();

        let evaluate_expr = |instr, index| match instr {
            Instruction::Call(_) => Instruction::Call(index),
            _ => unreachable!("Unexpected expr: {:?}", instr),
        };

        for instr in self.instructions.iter_mut() {
            match instr {
                Instruction::LabelRef(name, i) => {
                    let index = labels.get(name).unwrap();

                    *instr = evaluate_expr(*i.clone(), *index);
                }
                Instruction::Label(_, i) => {
                    *instr = *i.clone();
                }
                _ => (),
            }
        }
    }

    fn parse_functions(&mut self) -> Result<Vec<Instruction>, ParserError> {
        let mut parsed_instructions = Vec::new();
        for (_, tree) in self.functions.iter() {
            let mut instructions = self.parse_tree(tree.clone())?;

            parsed_instructions.append(&mut instructions);
        }

        Ok(parsed_instructions)
    }

    fn get_condition<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Vec<Token>, ParserError>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut values: Vec<Token> = Vec::new();

        while let Some(&t) = tokens.peek() {
            match t.token_type {
                TokenType::LeftBrace => {
                    break;
                }
                _ => {
                    tokens.next();
                    values.push(t.clone());
                }
            }
        }

        return Ok(values);
    }

    fn get_types<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Vec<Types>, ParserError>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut values: Vec<Types> = Vec::new();

        while let Some(&t) = tokens.peek() {
            match &t.token_type {
                TokenType::Type(t2) => {
                    values.push(t2.clone());
                }
                TokenType::LeftBrace => {
                    break;
                }
                TokenType::Arrow => {
                    break;
                }
                _ => {
                    return Err(ParserError::ExpectedTokenGot(
                        t.position_info.clone(),
                        TokenType::Type(Types::Void),
                        t.token_type.clone(),
                    ));
                }
            }
            tokens.next();
        }

        return Ok(values);
    }

    fn get_region<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Vec<Token>, ParserError>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut values: Vec<Token> = Vec::new();

        let mut count = 0;

        while let Some(&t) = tokens.peek() {
            match t.token_type {
                TokenType::LeftBrace => {
                    count += 1;
                    if count == 1 {
                        tokens.next();
                        continue;
                    }
                }
                TokenType::RightBrace => {
                    count -= 1;
                    if count == 0 {
                        tokens.next();
                        break;
                    }
                }
                _ => {}
            }
            values.push(t.clone());
            tokens.next();
        }

        return Ok(values);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_non_function(
        input: Vec<Token>,
        expected_tree: ParseTree,
        expected_instructions: Vec<Instruction>,
    ) {
        let mut parser = Parser::init(Config::blank(), input);
        match parser.parse() {
            Ok(_) => {
                assert_eq!(format!("{:?}", parser.functions), "{}");
                assert_eq!(
                    format!("{:?}", parser.parse_tree),
                    format!("{:?}", expected_tree)
                );
                assert_eq!(
                    format!("{:?}", parser.instructions),
                    format!("{:?}", expected_instructions)
                );
            }
            Err(e) => {
                assert!(false, "{:?}", e);
            }
        }
    }

    fn test_function(
        input: Vec<Token>,
        expected_function: HashMap<String, ParseTree>,
        expected_tree: ParseTree,
        expected_instructions: Vec<Instruction>,
    ) {
        let mut parser = Parser::init(Config::blank(), input);
        match parser.parse() {
            Ok(_) => {
                assert_eq!(
                    format!("{:?}", parser.functions),
                    format!("{:?}", expected_function)
                );
                assert_eq!(
                    format!("{:?}", parser.parse_tree),
                    format!("{:?}", expected_tree)
                );
                assert_eq!(
                    format!("{:?}", parser.instructions),
                    format!("{:?}", expected_instructions)
                );
            }
            Err(e) => {
                assert!(false, "{:?}", e);
            }
        }
    }

    #[test]
    fn parse_tree_normal() {
        let input = vec![
            Token::new(TokenType::I32(0), 1, 1, "0"),
            Token::new(TokenType::I32(1), 1, 3, "1"),
            Token::new(TokenType::Add, 1, 4, "+"),
            Token::new(TokenType::I32(2), 1, 5, "2"),
            Token::new(TokenType::I32(1), 1, 7, "1"),
            Token::new(TokenType::Subtract, 1, 8, "-"),
            Token::new(TokenType::Multiply, 1, 9, "*"),
        ];
        let expected_tree = ParseTree::Region(vec![
            ParseTree::Element(Token::new(TokenType::I32(0), 1, 1, "0")),
            ParseTree::Element(Token::new(TokenType::I32(1), 1, 3, "1")),
            ParseTree::Element(Token::new(TokenType::Add, 1, 4, "+")),
            ParseTree::Element(Token::new(TokenType::I32(2), 1, 5, "2")),
            ParseTree::Element(Token::new(TokenType::I32(1), 1, 7, "1")),
            ParseTree::Element(Token::new(TokenType::Subtract, 1, 8, "-")),
            ParseTree::Element(Token::new(TokenType::Multiply, 1, 9, "*")),
        ]);
        let expected_instructions = vec![
            Instruction::Push(StackValue::I32(0)),
            Instruction::Push(StackValue::I32(1)),
            Instruction::Add,
            Instruction::Push(StackValue::I32(2)),
            Instruction::Push(StackValue::I32(1)),
            Instruction::Subtract,
            Instruction::Multiply,
            Instruction::Halt,
        ];
        test_non_function(input, expected_tree, expected_instructions);
    }

    #[test]
    fn parse_tree_if() {
        let input = vec![
            Token::new(TokenType::I32(0), 1, 1, ""),
            Token::new(TokenType::If, 1, 1, ""),
            Token::new(TokenType::I32(10), 1, 1, ""),
            Token::new(TokenType::Greater, 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
        ];
        let expected_tree = ParseTree::Region(vec![
            ParseTree::Element(Token::new(TokenType::I32(0), 1, 1, "")),
            ParseTree::If(
                vec![(
                    Token::new(TokenType::If, 1, 1, ""),
                    Box::new(ParseTree::Region(vec![
                        ParseTree::Element(Token::new(TokenType::I32(10), 1, 1, "")),
                        ParseTree::Element(Token::new(TokenType::Greater, 1, 1, "")),
                    ])),
                    Box::new(ParseTree::Region(vec![])),
                )],
                (
                    Token::new(TokenType::If, 1, 1, ""),
                    Box::new(ParseTree::Region(vec![])),
                ),
            ),
        ]);
        let expected_instructions = vec![
            Instruction::Push(StackValue::I32(0)),
            Instruction::Push(StackValue::I32(10)),
            Instruction::Greater,
            Instruction::CondJump(1, 1),
            Instruction::Halt,
        ];
        test_non_function(input, expected_tree, expected_instructions);
    }

    #[test]
    fn parse_tree_if_else() {
        let input = vec![
            Token::new(TokenType::I32(0), 1, 1, ""),
            Token::new(TokenType::If, 1, 1, ""),
            Token::new(TokenType::I32(10), 1, 1, ""),
            Token::new(TokenType::Greater, 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            Token::new(TokenType::I32(2), 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
            Token::new(TokenType::Else, 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            Token::new(TokenType::I32(3), 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
        ];
        let expected_tree = ParseTree::Region(vec![
            ParseTree::Element(Token::new(TokenType::I32(0), 1, 1, "")),
            ParseTree::If(
                vec![(
                    Token::new(TokenType::If, 1, 1, ""),
                    Box::new(ParseTree::Region(vec![
                        ParseTree::Element(Token::new(TokenType::I32(10), 1, 1, "")),
                        ParseTree::Element(Token::new(TokenType::Greater, 1, 1, "")),
                    ])),
                    Box::new(ParseTree::Region(vec![ParseTree::Element(Token::new(
                        TokenType::I32(2),
                        1,
                        1,
                        "",
                    ))])),
                )],
                (
                    Token::new(TokenType::Else, 1, 1, ""),
                    Box::new(ParseTree::Region(vec![ParseTree::Element(Token::new(
                        TokenType::I32(3),
                        1,
                        1,
                        "",
                    ))])),
                ),
            ),
        ]);
        let expected_instructions = vec![
            Instruction::Push(StackValue::I32(0)),
            Instruction::Push(StackValue::I32(10)),
            Instruction::Greater,
            Instruction::CondJump(1, 3),
            Instruction::Push(StackValue::I32(2)),
            Instruction::Jump(2),
            Instruction::Push(StackValue::I32(3)),
            Instruction::Halt,
        ];
        test_non_function(input, expected_tree, expected_instructions);
    }

    #[test]
    fn parse_tree_if_elseif_else() {
        let input = vec![
            Token::new(TokenType::I32(0), 1, 1, ""),
            Token::new(TokenType::If, 1, 1, ""),
            Token::new(TokenType::Duplicate, 1, 1, ""),
            Token::new(TokenType::I32(10), 1, 1, ""),
            Token::new(TokenType::Greater, 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            Token::new(TokenType::I32(2), 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
            Token::new(TokenType::Else, 1, 1, ""),
            Token::new(TokenType::If, 1, 1, ""),
            Token::new(TokenType::Duplicate, 1, 1, ""),
            Token::new(TokenType::I32(20), 1, 1, ""),
            Token::new(TokenType::Greater, 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            Token::new(TokenType::I32(3), 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
            Token::new(TokenType::Else, 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            // Token::new(TokenType::Drop, 1, 1, ""),
            Token::new(TokenType::I32(4), 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
        ];
        let expected_tree = ParseTree::Region(vec![
            ParseTree::Element(Token::new(TokenType::I32(0), 1, 1, "")),
            ParseTree::If(
                vec![
                    (
                        Token::new(TokenType::If, 1, 1, ""),
                        Box::new(ParseTree::Region(vec![
                            ParseTree::Element(Token::new(TokenType::Duplicate, 1, 1, "")),
                            ParseTree::Element(Token::new(TokenType::I32(10), 1, 1, "")),
                            ParseTree::Element(Token::new(TokenType::Greater, 1, 1, "")),
                        ])),
                        Box::new(ParseTree::Region(vec![ParseTree::Element(Token::new(
                            TokenType::I32(2),
                            1,
                            1,
                            "",
                        ))])),
                    ),
                    (
                        Token::new(TokenType::If, 1, 1, ""),
                        Box::new(ParseTree::Region(vec![
                            ParseTree::Element(Token::new(TokenType::Duplicate, 1, 1, "")),
                            ParseTree::Element(Token::new(TokenType::I32(20), 1, 1, "")),
                            ParseTree::Element(Token::new(TokenType::Greater, 1, 1, "")),
                        ])),
                        Box::new(ParseTree::Region(vec![ParseTree::Element(Token::new(
                            TokenType::I32(3),
                            1,
                            1,
                            "",
                        ))])),
                    ),
                ],
                (
                    Token::new(TokenType::Else, 1, 1, ""),
                    Box::new(ParseTree::Region(vec![ParseTree::Element(Token::new(
                        TokenType::I32(4),
                        1,
                        1,
                        "",
                    ))])),
                ),
            ),
        ]);
        let expected_instructions = vec![
            Instruction::Push(StackValue::I32(0)),
            Instruction::Duplicate,
            Instruction::Push(StackValue::I32(10)),
            Instruction::Greater,
            Instruction::CondJump(1, 3),
            Instruction::Push(StackValue::I32(2)),
            Instruction::Jump(8),
            Instruction::Duplicate,
            Instruction::Push(StackValue::I32(20)),
            Instruction::Greater,
            Instruction::CondJump(1, 3),
            Instruction::Push(StackValue::I32(3)),
            Instruction::Jump(2),
            Instruction::Push(StackValue::I32(4)),
            Instruction::Halt,
        ];
        test_non_function(input, expected_tree, expected_instructions);
    }

    #[test]
    fn parse_tree_while() {
        let input = vec![
            Token::new(TokenType::I32(0), 1, 1, ""),
            Token::new(TokenType::While, 1, 1, ""),
            Token::new(TokenType::Duplicate, 1, 1, ""),
            Token::new(TokenType::I32(10), 1, 1, ""),
            Token::new(TokenType::Less, 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            Token::new(TokenType::I32(1), 1, 1, ""),
            Token::new(TokenType::Add, 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
        ];
        let expected_tree = ParseTree::Region(vec![
            ParseTree::Element(Token::new(TokenType::I32(0), 1, 1, "")),
            ParseTree::While(
                Token::new(TokenType::While, 1, 1, ""),
                Box::new(ParseTree::Region(vec![
                    ParseTree::Element(Token::new(TokenType::Duplicate, 1, 1, "")),
                    ParseTree::Element(Token::new(TokenType::I32(10), 1, 1, "")),
                    ParseTree::Element(Token::new(TokenType::Less, 1, 1, "")),
                ])),
                Box::new(ParseTree::Region(vec![
                    ParseTree::Element(Token::new(TokenType::I32(1), 1, 1, "")),
                    ParseTree::Element(Token::new(TokenType::Add, 1, 1, "")),
                ])),
            ),
        ]);
        let expected_instructions = vec![
            Instruction::Push(StackValue::I32(0)),
            Instruction::Duplicate,
            Instruction::Push(StackValue::I32(10)),
            Instruction::Less,
            Instruction::CondJump(1, 4),
            Instruction::Push(StackValue::I32(1)),
            Instruction::Add,
            Instruction::Jump(-6),
            Instruction::Halt,
        ];
        test_non_function(input, expected_tree, expected_instructions);
    }

    #[test]
    fn parse_tree_function() {
        let input = vec![
            Token::new(TokenType::Func, 1, 1, ""),
            Token::new(TokenType::Identifier("test".to_string()), 1, 1, ""),
            Token::new(TokenType::Type(Types::I32), 1, 1, ""),
            Token::new(TokenType::Type(Types::I32), 1, 1, ""),
            Token::new(TokenType::Arrow, 1, 1, ""),
            Token::new(TokenType::Type(Types::I32), 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            Token::new(TokenType::Add, 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
            Token::new(TokenType::I32(0), 1, 1, ""),
            Token::new(TokenType::I32(1), 1, 1, ""),
            Token::new(TokenType::Identifier("test".to_string()), 1, 1, ""),
            Token::new(TokenType::Print, 1, 1, ""),
        ];

        let expected_function = HashMap::from([(
            "test".to_string(),
            ParseTree::FuncDecl(
                Token::new(TokenType::Func, 1, 1, ""),
                "test".to_string(),
                vec![Types::I32, Types::I32],
                vec![Types::I32],
                Box::new(ParseTree::Region(vec![ParseTree::Element(Token::new(
                    TokenType::Add,
                    1,
                    1,
                    "",
                ))])),
            ),
        )]);

        let expected_tree = ParseTree::Region(vec![
            ParseTree::Element(Token::new(TokenType::I32(0), 1, 1, "")),
            ParseTree::Element(Token::new(TokenType::I32(1), 1, 1, "")),
            ParseTree::Element(Token::new(
                TokenType::Identifier("test".to_string()),
                1,
                1,
                "",
            )),
            ParseTree::Element(Token::new(TokenType::Print, 1, 1, "")),
        ]);

        let expected_instructions = vec![
            Instruction::Push(StackValue::I32(0)),
            Instruction::Push(StackValue::I32(1)),
            Instruction::Push(StackValue::I32(2)),
            Instruction::Call(6),
            Instruction::Print,
            Instruction::Halt,
            Instruction::Add,
            Instruction::Push(StackValue::I32(1)),
            Instruction::Ret,
        ];

        test_function(
            input,
            expected_function,
            expected_tree,
            expected_instructions,
        );
    }
}
