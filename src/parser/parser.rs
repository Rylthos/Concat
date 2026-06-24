use crate::error::types::{ErrorType, ParserError};
use crate::lexer::tokens::{Token, TokenType};
use crate::parser::instructions::Instruction;
use crate::parser::intrinsics::Intrinsic;
use crate::parser::parse_tree::{FuncDecl, ParseTree};
use crate::parser::stack_types::StackType;
use crate::parser::stack_values::StackValue;
use crate::parser::typing::Typing;

use crate::config::config::Config;

use std::collections::HashMap;

pub struct Parser {
    config: Config,

    tokens: Vec<Token>,
    parse_tree: ParseTree,

    functions: HashMap<String, FuncDecl>,

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
        let tree =
            match ParseTree::generate_parse_tree(tokens.iter().peekable(), &mut self.functions) {
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

        let mut list = match self.parse_tree(self.parse_tree.clone(), &HashMap::new()) {
            Ok(i) => i,
            Err(e) => return Err(ErrorType::Parser(e)),
        };
        list.push(Intrinsic::Halt);

        let mut function_instructions = match self.parse_functions() {
            Ok(i) => i,
            Err(e) => return Err(ErrorType::Parser(e)),
        };
        list.append(&mut function_instructions);

        self.instructions = match Self::generate_instructions(&list) {
            Ok(i) => i,
            Err(e) => return Err(ErrorType::Parser(e)),
        };

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

    pub fn convert_token(token: &Token) -> Intrinsic {
        match token.token_type.clone() {
            TokenType::LeftBrace
            | TokenType::RightBrace
            | TokenType::If
            | TokenType::Else
            | TokenType::While
            | TokenType::Arrow
            | TokenType::Func
            | TokenType::Assignment
            | TokenType::Void
            | TokenType::I32
            | TokenType::String
            | TokenType::Bool => {
                unreachable!("Unreachable: {:?}", token)
            }
            TokenType::StringValue(s) => Intrinsic::StringValue(s.clone()),
            TokenType::I32Value(n) => Intrinsic::I32Value(n),
            TokenType::BoolValue(b) => Intrinsic::BoolValue(b),
            TokenType::Identifier(s) => Intrinsic::Identifier(s),
            //
            TokenType::Add => Intrinsic::Add,
            TokenType::Subtract => Intrinsic::Subtract,
            TokenType::Asterisk => Intrinsic::Multiply,
            TokenType::Divide => Intrinsic::Divide,
            TokenType::Modulo => Intrinsic::Modulo,
            //
            TokenType::Rotate3 => Intrinsic::Rotate3,
            TokenType::Duplicate => Intrinsic::Duplicate,
            TokenType::Drop => Intrinsic::Drop,
            TokenType::Over => Intrinsic::Over,
            TokenType::Swap => Intrinsic::Swap,
            TokenType::Print => Intrinsic::Print,
            //
            TokenType::Less => Intrinsic::Less,
            TokenType::LessEqual => Intrinsic::LessEqual,
            TokenType::Greater => Intrinsic::Greater,
            TokenType::GreaterEqual => Intrinsic::GreaterEqual,
            TokenType::Equal => Intrinsic::Equal,
            TokenType::NotEqual => Intrinsic::NotEqual,
            TokenType::And => Intrinsic::And,
            TokenType::Or => Intrinsic::Or,
            TokenType::Not => Intrinsic::Not,
            //
            TokenType::Read => Intrinsic::Read,
            TokenType::Assign => Intrinsic::Assign,
            //
            TokenType::Mem => Intrinsic::Mem,
            //
            TokenType::DebugPrintStack => Intrinsic::DebugPrintStack,
        }
    }

    fn convert_intrinsic(intrinsic: &Intrinsic, labels: &HashMap<String, usize>) -> Instruction {
        match intrinsic {
            Intrinsic::Add => Instruction::Add,
            Intrinsic::Subtract => Instruction::Subtract,
            Intrinsic::Multiply => Instruction::Multiply,
            Intrinsic::Divide => Instruction::Divide,
            Intrinsic::Modulo => Instruction::Modulo,

            Intrinsic::Rotate3 => Instruction::Rotate3,
            Intrinsic::Duplicate => Instruction::Duplicate,
            Intrinsic::Drop => Instruction::Drop,
            Intrinsic::Over => Instruction::Over,
            Intrinsic::Swap => Instruction::Swap,
            Intrinsic::Print => Instruction::Print,

            Intrinsic::Less => Instruction::Less,
            Intrinsic::Greater => Instruction::Greater,
            Intrinsic::LessEqual => Instruction::LessEqual,
            Intrinsic::GreaterEqual => Instruction::GreaterEqual,
            Intrinsic::Equal => Instruction::Equal,
            Intrinsic::NotEqual => Instruction::NotEqual,
            Intrinsic::And => Instruction::And,
            Intrinsic::Or => Instruction::Or,
            Intrinsic::Not => Instruction::Not,

            Intrinsic::Assign => Instruction::Assign,
            Intrinsic::Read => Instruction::Read,
            Intrinsic::Lookup(d, s) => Instruction::Lookup(*d, *s),

            Intrinsic::Jump(j) => Instruction::Jump(*j),
            Intrinsic::CondJump(t, f) => Instruction::CondJump(*t, *f),

            Intrinsic::Identifier(_) => todo!(),

            Intrinsic::Mem => Instruction::Mem,
            Intrinsic::Ret => Instruction::Ret,
            Intrinsic::Call(_) => unreachable!(),

            Intrinsic::StackType(t) => Instruction::Push(StackValue::Type(t.clone())),
            Intrinsic::StringValue(s) => Instruction::Push(StackValue::String(s.clone())),
            Intrinsic::I32Value(i) => Instruction::Push(StackValue::I32(*i)),
            Intrinsic::BoolValue(b) => Instruction::Push(StackValue::Bool(*b)),

            Intrinsic::FrameCreate => Instruction::FrameCreate,
            Intrinsic::FrameRemove => Instruction::FrameRemove,

            Intrinsic::FuncLabelDecl(_, intrinsic) => Self::convert_intrinsic(intrinsic, labels),
            Intrinsic::FuncLabelRef(func_name, intrinsic) => match **intrinsic {
                Intrinsic::Call(_) => Instruction::Call(*labels.get(func_name).unwrap()),
                _ => unreachable!(),
            },

            Intrinsic::DebugPrintStack => Instruction::DebugPrintStack,

            Intrinsic::Halt => Instruction::Halt,
        }
    }

    fn parse_tree<'a>(
        &self,
        tree: ParseTree,
        variable_lookup: &HashMap<String, (usize, usize)>,
    ) -> Result<Vec<Intrinsic>, ParserError> {
        let mut parsed_expression: Vec<Intrinsic> = Vec::new();

        match tree {
            ParseTree::None => return Err(ParserError::InvalidParseTree()),
            ParseTree::Element(p, i) => match i {
                Intrinsic::Identifier(iden) => {
                    if let Some(func) = self.functions.get(&iden) {
                        parsed_expression.push(Intrinsic::I32Value(func.inputs.len() as i32));
                        parsed_expression
                            .push(Intrinsic::FuncLabelRef(iden, Box::new(Intrinsic::Call(0))));
                    } else if let Some((d, s)) = variable_lookup.get(&iden) {
                        parsed_expression.push(Intrinsic::Lookup(*d, *s));
                    } else {
                        return Err(ParserError::UnknownIdentifier(p, iden.to_string()));
                    }
                }
                _ => {
                    parsed_expression.push(i);
                }
            },
            ParseTree::Region(r) => parsed_expression.append(
                &mut r
                    .iter()
                    .map(|m| match self.parse_tree(m.clone(), variable_lookup) {
                        Ok(r) => r,
                        Err(_) => panic!(),
                    })
                    .flatten()
                    .collect::<Vec<Intrinsic>>(),
            ),
            ParseTree::If(if_branches, (_, else_branch)) => {
                let if_branches_result = if_branches
                    .iter()
                    .map(|(_, c, m)| {
                        let c1 = self.parse_tree(*c.clone(), variable_lookup);
                        let c2 = self.parse_tree(*m.clone(), variable_lookup);
                        (c1, c2)
                    })
                    .collect::<Vec<(
                        Result<Vec<Intrinsic>, ParserError>,
                        Result<Vec<Intrinsic>, ParserError>,
                    )>>();

                let mut if_branches = Vec::new();
                for (c, r) in if_branches_result {
                    let c = c?;
                    let r = r?;
                    if_branches.push((c, r));
                }

                let mut else_branch = self.parse_tree(*else_branch, variable_lookup)?;

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
                    parsed_expression.push(Intrinsic::CondJump(
                        1,
                        r.len() + 1 + if jump_length != 0 { 1 } else { 0 },
                    ));
                    parsed_expression.append(&mut r);

                    if jump_length != 0 {
                        length_seen += 1;
                        parsed_expression.push(Intrinsic::Jump(
                            (jump_length + (total_conditional_branches - branches_seen)) as isize,
                        ));
                    }

                    branches_seen += 1;
                }

                parsed_expression.append(&mut else_branch);
            }
            ParseTree::While(_, c, r) => {
                let mut condition_tree = self.parse_tree(*c, variable_lookup)?;
                let mut region_tree = self.parse_tree(*r, variable_lookup)?;

                let total_length = condition_tree.len() + region_tree.len();
                parsed_expression.append(&mut condition_tree);
                parsed_expression.push(Intrinsic::CondJump(1, region_tree.len() + 2));
                parsed_expression.append(&mut region_tree);
                parsed_expression.push(Intrinsic::Jump(-(total_length as isize) - 1));
            }
            ParseTree::Assign(_, v, r) => {
                let mut lookup = variable_lookup.clone();

                for (_, (d, _)) in lookup.iter_mut() {
                    *d += 1
                }

                let mut slot = 0;
                for v in v.iter() {
                    lookup.insert(v.to_string(), (0, slot));
                    slot += 1;
                }

                let mut region_tree = self.parse_tree(*r, &lookup)?;

                parsed_expression.push(Intrinsic::I32Value(v.len() as i32));
                parsed_expression.push(Intrinsic::FrameCreate);

                parsed_expression.append(&mut region_tree);
                parsed_expression.push(Intrinsic::I32Value(v.len() as i32));
                parsed_expression.push(Intrinsic::FrameRemove);
            }
            ParseTree::FuncDecl(func) => {
                let mut region = self.parse_tree(*func.region, variable_lookup)?;

                region.push(Intrinsic::I32Value(func.outputs.len() as i32));
                region.push(Intrinsic::Ret);
                let initial_token = region.get(0).unwrap();
                *region.get_mut(0).unwrap() =
                    Intrinsic::FuncLabelDecl(func.name, Box::new(initial_token.clone()));

                parsed_expression.append(&mut region);
            }
        }

        return Ok(parsed_expression);
    }

    fn generate_instructions(list: &Vec<Intrinsic>) -> Result<Vec<Instruction>, ParserError> {
        let labels: HashMap<String, usize> = list
            .iter()
            .zip(0..)
            .filter(|(instr, _)| match instr {
                Intrinsic::FuncLabelDecl(_, _) => true,
                _ => false,
            })
            .map(|(instr, i)| match instr {
                Intrinsic::FuncLabelDecl(name, _) => (name.to_string(), i),
                _ => unreachable!(),
            })
            .collect();

        let mut instructions = Vec::new();

        for intrinsic in list.iter() {
            instructions.push(Self::convert_intrinsic(intrinsic, &labels))
        }

        Ok(instructions)
    }

    fn parse_functions(&mut self) -> Result<Vec<Intrinsic>, ParserError> {
        let mut parsed_instructions = Vec::new();
        for (_, func) in self.functions.iter() {
            let mut instructions = self.parse_tree(*func.region.clone(), &HashMap::new())?;

            instructions.push(Intrinsic::Ret);
            let first_instr = instructions.get(0).unwrap().clone();
            *instructions.get_mut(0).unwrap() =
                Intrinsic::FuncLabelDecl(func.name.clone(), Box::new(first_instr));

            parsed_instructions.append(&mut instructions);
        }

        Ok(parsed_instructions)
    }

    pub fn get_condition<'a, I>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Vec<Token>, ParserError>
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

    pub fn get_types<'a, I>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Vec<StackType>, ParserError>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut values: Vec<StackType> = Vec::new();

        let mut should_end_next = false;

        while let Some(&t) = tokens.peek() {
            let check_end = should_end_next;
            match &t.token_type {
                TokenType::String | TokenType::I32 | TokenType::Bool => {
                    values.push(StackType::convert_type(&t.token_type));
                }
                TokenType::Asterisk => {
                    if let Some(top_value) = values.pop() {
                        values.push(StackType::Ptr(Box::new(top_value)));
                    } else {
                        return Err(ParserError::ExpectedTypeGot(
                            t.position_info.clone(),
                            t.token_type.clone(),
                        ));
                    }
                }
                TokenType::Void => {
                    should_end_next = true;
                }
                TokenType::LeftBrace => {
                    break;
                }
                TokenType::Arrow => {
                    break;
                }
                _ => {
                    return Err(ParserError::ExpectedTypeGot(
                        t.position_info.clone(),
                        t.token_type.clone(),
                    ));
                }
            }
            tokens.next();

            if check_end {
                todo!("Should not allow for more types after void");
            }
        }

        return Ok(values);
    }

    pub fn get_identifier_list<'a, I>(
        tokens: &mut std::iter::Peekable<I>,
    ) -> Result<Vec<Token>, ParserError>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut identifiers: Vec<Token> = Vec::new();
        while let Some(&t) = tokens.peek() {
            match t.token_type {
                TokenType::LeftBrace => {
                    break;
                }
                TokenType::Identifier(_) => {
                    identifiers.push(t.clone());
                }
                _ => {
                    return Err(ParserError::ExpectedIdentifierGot(
                        t.position_info.clone(),
                        t.token_type.clone(),
                    ));
                }
            }
            tokens.next();
        }

        Ok(identifiers)
    }

    pub fn get_region<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Vec<Token>, ParserError>
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
    use crate::lexer::tokens::PositionInfo;

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
        expected_function: HashMap<String, FuncDecl>,
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

    fn create_element(line: usize, column: usize, intrinsic: Intrinsic) -> ParseTree {
        ParseTree::Element(PositionInfo { line, column }, intrinsic)
    }

    #[test]
    fn parse_tree_normal() {
        let input = vec![
            Token::new(TokenType::I32Value(0), 1, 1, "0"),
            Token::new(TokenType::I32Value(1), 1, 3, "1"),
            Token::new(TokenType::Add, 1, 4, "+"),
            Token::new(TokenType::I32Value(2), 1, 5, "2"),
            Token::new(TokenType::I32Value(1), 1, 7, "1"),
            Token::new(TokenType::Subtract, 1, 8, "-"),
            Token::new(TokenType::Asterisk, 1, 9, "*"),
        ];
        let expected_tree = ParseTree::Region(vec![
            create_element(1, 1, Intrinsic::I32Value(0)),
            create_element(1, 3, Intrinsic::I32Value(1)),
            create_element(1, 4, Intrinsic::Add),
            create_element(1, 5, Intrinsic::I32Value(2)),
            create_element(1, 7, Intrinsic::I32Value(1)),
            create_element(1, 8, Intrinsic::Subtract),
            create_element(1, 9, Intrinsic::Multiply),
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
            Token::new(TokenType::I32Value(0), 1, 1, ""),
            Token::new(TokenType::If, 1, 1, ""),
            Token::new(TokenType::I32Value(10), 1, 1, ""),
            Token::new(TokenType::Greater, 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
        ];
        let expected_tree = ParseTree::Region(vec![
            create_element(1, 1, Intrinsic::I32Value(0)),
            ParseTree::If(
                vec![(
                    Token::new(TokenType::If, 1, 1, ""),
                    Box::new(ParseTree::Region(vec![
                        create_element(1, 1, Intrinsic::I32Value(10)),
                        create_element(1, 1, Intrinsic::Greater),
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
            Token::new(TokenType::I32Value(0), 1, 1, ""),
            Token::new(TokenType::If, 1, 1, ""),
            Token::new(TokenType::I32Value(10), 1, 1, ""),
            Token::new(TokenType::Greater, 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            Token::new(TokenType::I32Value(2), 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
            Token::new(TokenType::Else, 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            Token::new(TokenType::I32Value(3), 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
        ];
        let expected_tree = ParseTree::Region(vec![
            create_element(1, 1, Intrinsic::I32Value(0)),
            ParseTree::If(
                vec![(
                    Token::new(TokenType::If, 1, 1, ""),
                    Box::new(ParseTree::Region(vec![
                        create_element(1, 1, Intrinsic::I32Value(10)),
                        create_element(1, 1, Intrinsic::Greater),
                    ])),
                    Box::new(ParseTree::Region(vec![create_element(
                        1,
                        1,
                        Intrinsic::I32Value(2),
                    )])),
                )],
                (
                    Token::new(TokenType::Else, 1, 1, ""),
                    Box::new(ParseTree::Region(vec![create_element(
                        1,
                        1,
                        Intrinsic::I32Value(3),
                    )])),
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
            Token::new(TokenType::I32Value(0), 1, 1, ""),
            Token::new(TokenType::If, 1, 1, ""),
            Token::new(TokenType::Duplicate, 1, 1, ""),
            Token::new(TokenType::I32Value(10), 1, 1, ""),
            Token::new(TokenType::Greater, 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            Token::new(TokenType::I32Value(2), 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
            Token::new(TokenType::Else, 1, 1, ""),
            Token::new(TokenType::If, 1, 1, ""),
            Token::new(TokenType::Duplicate, 1, 1, ""),
            Token::new(TokenType::I32Value(20), 1, 1, ""),
            Token::new(TokenType::Greater, 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            Token::new(TokenType::I32Value(3), 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
            Token::new(TokenType::Else, 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            // Token::new(TokenType::Drop, 1, 1, ""),
            Token::new(TokenType::I32Value(4), 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
        ];
        let expected_tree = ParseTree::Region(vec![
            create_element(1, 1, Intrinsic::I32Value(0)),
            ParseTree::If(
                vec![
                    (
                        Token::new(TokenType::If, 1, 1, ""),
                        Box::new(ParseTree::Region(vec![
                            create_element(1, 1, Intrinsic::Duplicate),
                            create_element(1, 1, Intrinsic::I32Value(10)),
                            create_element(1, 1, Intrinsic::Greater),
                        ])),
                        Box::new(ParseTree::Region(vec![create_element(
                            1,
                            1,
                            Intrinsic::I32Value(2),
                        )])),
                    ),
                    (
                        Token::new(TokenType::If, 1, 1, ""),
                        Box::new(ParseTree::Region(vec![
                            create_element(1, 1, Intrinsic::Duplicate),
                            create_element(1, 1, Intrinsic::I32Value(20)),
                            create_element(1, 1, Intrinsic::Greater),
                        ])),
                        Box::new(ParseTree::Region(vec![create_element(
                            1,
                            1,
                            Intrinsic::I32Value(3),
                        )])),
                    ),
                ],
                (
                    Token::new(TokenType::Else, 1, 1, ""),
                    Box::new(ParseTree::Region(vec![create_element(
                        1,
                        1,
                        Intrinsic::I32Value(4),
                    )])),
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
            Token::new(TokenType::I32Value(0), 1, 1, ""),
            Token::new(TokenType::While, 1, 1, ""),
            Token::new(TokenType::Duplicate, 1, 1, ""),
            Token::new(TokenType::I32Value(10), 1, 1, ""),
            Token::new(TokenType::Less, 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            Token::new(TokenType::I32Value(1), 1, 1, ""),
            Token::new(TokenType::Add, 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
        ];
        let expected_tree = ParseTree::Region(vec![
            create_element(1, 1, Intrinsic::I32Value(0)),
            ParseTree::While(
                Token::new(TokenType::While, 1, 1, ""),
                Box::new(ParseTree::Region(vec![
                    create_element(1, 1, Intrinsic::Duplicate),
                    create_element(1, 1, Intrinsic::I32Value(10)),
                    create_element(1, 1, Intrinsic::Less),
                ])),
                Box::new(ParseTree::Region(vec![
                    create_element(1, 1, Intrinsic::I32Value(1)),
                    create_element(1, 1, Intrinsic::Add),
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
    fn parse_ptr() {
        let input = vec![
            Token::new(TokenType::I32, 1, 1, "i32"),
            Token::new(TokenType::Asterisk, 1, 2, "*"),
        ];
        let expected_tree = ParseTree::Region(vec![create_element(
            1,
            1,
            Intrinsic::StackType(StackType::Ptr(Box::new(StackType::I32))),
        )]);
        let expected_instructions = vec![
            Instruction::Push(StackValue::Type(StackType::Ptr(Box::new(StackType::I32)))),
            Instruction::Halt,
        ];
        test_non_function(input, expected_tree, expected_instructions);
    }

    #[test]
    fn parse_tree_function() {
        let input = vec![
            Token::new(TokenType::Func, 1, 1, ""),
            Token::new(TokenType::Identifier("test".to_string()), 1, 1, ""),
            Token::new(TokenType::I32, 1, 1, ""),
            Token::new(TokenType::I32, 1, 1, ""),
            Token::new(TokenType::Arrow, 1, 1, ""),
            Token::new(TokenType::I32, 1, 1, ""),
            Token::new(TokenType::LeftBrace, 1, 1, ""),
            Token::new(TokenType::Add, 1, 1, ""),
            Token::new(TokenType::RightBrace, 1, 1, ""),
            Token::new(TokenType::I32Value(0), 1, 1, ""),
            Token::new(TokenType::I32Value(1), 1, 1, ""),
            Token::new(TokenType::Identifier("test".to_string()), 1, 1, ""),
            Token::new(TokenType::Print, 1, 1, ""),
        ];

        let expected_function = HashMap::from([(
            "test".to_string(),
            FuncDecl {
                position_info: PositionInfo { line: 1, column: 1 },
                name: "test".to_string(),
                inputs: vec![StackType::I32, StackType::I32],
                outputs: vec![StackType::I32],
                region: Box::new(ParseTree::Region(vec![create_element(
                    1,
                    1,
                    Intrinsic::Add,
                )])),
            },
        )]);

        let expected_tree = ParseTree::Region(vec![
            create_element(1, 1, Intrinsic::I32Value(0)),
            create_element(1, 1, Intrinsic::I32Value(1)),
            create_element(1, 1, Intrinsic::Identifier("test".to_string())),
            create_element(1, 1, Intrinsic::Print),
        ]);

        let expected_instructions = vec![
            Instruction::Push(StackValue::I32(0)),
            Instruction::Push(StackValue::I32(1)),
            Instruction::Push(StackValue::I32(2)),
            Instruction::Call(6),
            Instruction::Print,
            Instruction::Halt,
            Instruction::Add,
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
