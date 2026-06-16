use crate::lexer::tokens::{Token, TokenType, Types};
use crate::parser::instructions::{Instruction, StackValue};

use std::collections::HashMap;

#[derive(Debug, Clone)]
enum ParseTree {
    Element(Token),
    Region(Vec<ParseTree>),
    If(Vec<(Box<ParseTree>, Box<ParseTree>)>, Box<ParseTree>),
    While(Box<ParseTree>, Box<ParseTree>),
    FuncDecl(String, Vec<Types>, Vec<Types>, Box<ParseTree>),
}

fn parse_element(token: Token) -> Option<Instruction> {
    let instr = match token.token_type {
        TokenType::LeftBrace
        | TokenType::RightBrace
        | TokenType::If
        | TokenType::Else
        | TokenType::While
        | TokenType::Identifier(_)
        | TokenType::Arrow
        | TokenType::Func => {
            panic!("Unreachable: {:?}", token);
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
    };

    return Some(instr);
}

fn parse_tree<'a>(tree: ParseTree) -> Result<Vec<Instruction>, String> {
    let mut parsed_expression: Vec<Instruction> = Vec::new();

    match tree {
        ParseTree::Element(e) => {
            let expr = parse_element(e);
            if let Some(e) = expr {
                parsed_expression.push(e)
            } else {
                panic!()
            }
        }
        ParseTree::Region(r) => parsed_expression.append(
            &mut r
                .iter()
                .map(|m| match parse_tree(m.clone()) {
                    Ok(r) => r,
                    Err(_) => panic!(),
                })
                .flatten()
                .collect::<Vec<Instruction>>(),
        ),
        ParseTree::If(if_branches, else_branch) => {
            let if_branches = if_branches
                .iter()
                .map(|(c, m)| {
                    let c1 = match parse_tree(*c.clone()) {
                        Ok(o) => o,
                        Err(e) => panic!("{}", e),
                    };
                    let c2 = match parse_tree(*m.clone()) {
                        Ok(o) => o,
                        Err(e) => panic!("{}", e),
                    };
                    (c1, c2)
                })
                .collect::<Vec<(Vec<Instruction>, Vec<Instruction>)>>();

            let mut else_branch = parse_tree(*else_branch)?;

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
                    0,
                    r.len() + if jump_length != 0 { 1 } else { 0 },
                ));
                parsed_expression.append(&mut r);

                if jump_length != 0 {
                    length_seen += 1;
                    parsed_expression.push(Instruction::Jump(
                        jump_length + (total_conditional_branches - branches_seen - 1),
                    ));
                }

                branches_seen += 1;
            }

            parsed_expression.append(&mut else_branch);
        }
        ParseTree::While(c, r) => {
            let mut condition_tree = parse_tree(*c)?;
            let mut region_tree = parse_tree(*r)?;

            let total_length = condition_tree.len() + region_tree.len();
            parsed_expression.append(&mut condition_tree);
            parsed_expression.push(Instruction::CondJump(0, region_tree.len() + 1));
            parsed_expression.append(&mut region_tree);
            parsed_expression.push(Instruction::BackJump(total_length + 1));
        }
        ParseTree::FuncDecl(name, inputs, outputs, region) => {
            todo!("Implement func passing");
        }
    }

    return Ok(parsed_expression);
}

fn get_condition<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Vec<Token>, String>
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

fn get_types<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Vec<Types>, String>
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
            _ => panic!("Invalid token, expected type got {:?}", t.token_type),
        }
        tokens.next();
    }

    return Ok(values);
}

fn get_region<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Vec<Token>, String>
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

fn generate_parse_tree<'a>(
    tokens: impl Iterator<Item = &'a Token>,
    functions: &mut HashMap<String, ParseTree>,
) -> Result<ParseTree, String> {
    let mut region: Vec<ParseTree> = Vec::new();
    let mut peekable = tokens.peekable();

    while let Some(&t) = peekable.peek() {
        match t.token_type {
            TokenType::If => {
                peekable.next();
                let mut regions = Vec::new();
                let mut else_region = ParseTree::Region(vec![]);

                loop {
                    let conditional_tree =
                        generate_parse_tree(get_condition(&mut peekable)?.iter(), functions)?;
                    let region_tree =
                        generate_parse_tree(get_region(&mut peekable)?.iter(), functions)?;

                    regions.push((Box::new(conditional_tree), Box::new(region_tree)));

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

                                else_region = generate_parse_tree(
                                    get_region(&mut peekable)?.iter(),
                                    functions,
                                )?;

                                break;
                            }
                            _ => break,
                        }
                    } else {
                        break;
                    }
                }

                region.push(ParseTree::If(regions, Box::new(else_region)));
            }
            TokenType::While => {
                peekable.next();
                let conditional_tree =
                    generate_parse_tree(get_condition(&mut peekable)?.iter(), functions)?;
                let region_tree =
                    generate_parse_tree(get_region(&mut peekable)?.iter(), functions)?;

                region.push(ParseTree::While(
                    Box::new(conditional_tree),
                    Box::new(region_tree),
                ))
            }
            TokenType::Func => {
                peekable.next();
                let function_name = if let Some(t) = peekable.next() {
                    match &t.token_type {
                        TokenType::Identifier(s) => s.clone(),
                        _ => panic!("Invalid function call"),
                    }
                } else {
                    panic!("Invalid function");
                };

                let input_types = get_types(&mut peekable)?;

                if let Some(t) = peekable.next() {
                    match &t.token_type {
                        TokenType::Arrow => (),
                        _ => panic!("Expected arrow operator"),
                    }
                } else {
                    panic!("Expected arrow");
                };

                let output_types = get_types(&mut peekable)?;

                let region_tree =
                    generate_parse_tree(get_region(&mut peekable)?.iter(), functions)?;

                functions.insert(
                    function_name.clone(),
                    ParseTree::FuncDecl(
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

fn parse(tokens: &Vec<Token>) -> Result<Vec<Instruction>, String> {
    let mut functions = HashMap::new();
    let result = generate_parse_tree(tokens.iter(), &mut functions);

    let tree = match result {
        Ok(t) => t,
        Err(e) => panic!("Invalid: {}", e),
    };

    let expression = parse_tree(tree);

    return expression;
}

pub fn parse_tokens(tokens: &Vec<Token>) -> Vec<Instruction> {
    let expr_result = parse(tokens);

    match expr_result {
        Ok(expr) => return expr,
        Err(s) => panic!("Parsing failed: {}", s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_non_function(input: Vec<Token>, expected: ParseTree) {
        let mut functions = HashMap::new();
        let tree = generate_parse_tree(input.iter(), &mut functions);

        match tree {
            Ok(t) => {
                assert_eq!(format!("{:?}", functions), "{}");
                assert_eq!(format!("{:?}", t), format!("{:?}", expected));
            }
            Err(e) => {
                assert!(false, "{}", e);
            }
        }
    }

    fn test_function(
        input: Vec<Token>,
        function_expected: HashMap<String, ParseTree>,
        expected: ParseTree,
    ) {
        let mut functions = HashMap::new();
        let tree = generate_parse_tree(input.iter(), &mut functions);

        match tree {
            Ok(t) => {
                assert_eq!(
                    format!("{:?}", functions),
                    format!("{:?}", function_expected)
                );
                assert_eq!(format!("{:?}", t), format!("{:?}", expected));
            }
            Err(e) => {
                assert!(false, "{}", e);
            }
        }
    }

    #[test]
    fn parse_tree_normal() {
        let input = vec![
            Token::new(TokenType::Add, 1),
            Token::new(TokenType::Subtract, 1),
            Token::new(TokenType::Multiply, 1),
        ];
        let expected = ParseTree::Region(vec![
            ParseTree::Element(Token::new(TokenType::Add, 1)),
            ParseTree::Element(Token::new(TokenType::Subtract, 1)),
            ParseTree::Element(Token::new(TokenType::Multiply, 1)),
        ]);
        test_non_function(input, expected);
    }

    #[test]
    fn parse_tree_if() {
        let input = vec![
            Token::new(TokenType::I32(0), 1),
            Token::new(TokenType::If, 1),
            Token::new(TokenType::I32(10), 1),
            Token::new(TokenType::Greater, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::RightBrace, 1),
        ];
        let expected = ParseTree::Region(vec![
            ParseTree::Element(Token::new(TokenType::I32(0), 1)),
            ParseTree::If(
                vec![(
                    Box::new(ParseTree::Region(vec![
                        ParseTree::Element(Token::new(TokenType::I32(10), 1)),
                        ParseTree::Element(Token::new(TokenType::Greater, 1)),
                    ])),
                    Box::new(ParseTree::Region(vec![])),
                )],
                Box::new(ParseTree::Region(vec![])),
            ),
        ]);
        test_non_function(input, expected);
    }

    #[test]
    fn parse_tree_if_else() {
        let input = vec![
            Token::new(TokenType::I32(0), 1),
            Token::new(TokenType::If, 1),
            Token::new(TokenType::I32(10), 1),
            Token::new(TokenType::Greater, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::I32(2), 1),
            Token::new(TokenType::RightBrace, 1),
            Token::new(TokenType::Else, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::I32(3), 1),
            Token::new(TokenType::RightBrace, 1),
        ];
        let expected = ParseTree::Region(vec![
            ParseTree::Element(Token::new(TokenType::I32(0), 1)),
            ParseTree::If(
                vec![(
                    Box::new(ParseTree::Region(vec![
                        ParseTree::Element(Token::new(TokenType::I32(10), 1)),
                        ParseTree::Element(Token::new(TokenType::Greater, 1)),
                    ])),
                    Box::new(ParseTree::Region(vec![ParseTree::Element(Token::new(
                        TokenType::I32(2),
                        1,
                    ))])),
                )],
                Box::new(ParseTree::Region(vec![ParseTree::Element(Token::new(
                    TokenType::I32(3),
                    1,
                ))])),
            ),
        ]);
        test_non_function(input, expected);
    }

    #[test]
    fn parse_tree_if_elseif_else() {
        let input = vec![
            Token::new(TokenType::I32(0), 1),
            Token::new(TokenType::If, 1),
            Token::new(TokenType::Duplicate, 1),
            Token::new(TokenType::I32(10), 1),
            Token::new(TokenType::Greater, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::I32(2), 1),
            Token::new(TokenType::RightBrace, 1),
            Token::new(TokenType::Else, 1),
            Token::new(TokenType::If, 1),
            Token::new(TokenType::Duplicate, 1),
            Token::new(TokenType::I32(20), 1),
            Token::new(TokenType::Greater, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::I32(3), 1),
            Token::new(TokenType::RightBrace, 1),
            Token::new(TokenType::Else, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::Drop, 1),
            Token::new(TokenType::I32(4), 1),
            Token::new(TokenType::RightBrace, 1),
        ];
        let expected = ParseTree::Region(vec![
            ParseTree::Element(Token::new(TokenType::I32(0), 1)),
            ParseTree::If(
                vec![
                    (
                        Box::new(ParseTree::Region(vec![
                            ParseTree::Element(Token::new(TokenType::Duplicate, 1)),
                            ParseTree::Element(Token::new(TokenType::I32(10), 1)),
                            ParseTree::Element(Token::new(TokenType::Greater, 1)),
                        ])),
                        Box::new(ParseTree::Region(vec![ParseTree::Element(Token::new(
                            TokenType::I32(2),
                            1,
                        ))])),
                    ),
                    (
                        Box::new(ParseTree::Region(vec![
                            ParseTree::Element(Token::new(TokenType::Duplicate, 1)),
                            ParseTree::Element(Token::new(TokenType::I32(20), 1)),
                            ParseTree::Element(Token::new(TokenType::Greater, 1)),
                        ])),
                        Box::new(ParseTree::Region(vec![ParseTree::Element(Token::new(
                            TokenType::I32(3),
                            1,
                        ))])),
                    ),
                ],
                Box::new(ParseTree::Region(vec![
                    ParseTree::Element(Token::new(TokenType::Drop, 1)),
                    ParseTree::Element(Token::new(TokenType::I32(4), 1)),
                ])),
            ),
        ]);
        test_non_function(input, expected);
    }
    #[test]
    fn parse_tree_while() {
        let input = vec![
            Token::new(TokenType::I32(0), 1),
            Token::new(TokenType::While, 1),
            Token::new(TokenType::Duplicate, 1),
            Token::new(TokenType::I32(10), 1),
            Token::new(TokenType::Less, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::I32(1), 1),
            Token::new(TokenType::Add, 1),
            Token::new(TokenType::RightBrace, 1),
        ];
        let expected = ParseTree::Region(vec![
            ParseTree::Element(Token::new(TokenType::I32(0), 1)),
            ParseTree::While(
                Box::new(ParseTree::Region(vec![
                    ParseTree::Element(Token::new(TokenType::Duplicate, 1)),
                    ParseTree::Element(Token::new(TokenType::I32(10), 1)),
                    ParseTree::Element(Token::new(TokenType::Less, 1)),
                ])),
                Box::new(ParseTree::Region(vec![
                    ParseTree::Element(Token::new(TokenType::I32(1), 1)),
                    ParseTree::Element(Token::new(TokenType::Add, 1)),
                ])),
            ),
        ]);
        test_non_function(input, expected);
    }

    #[test]
    fn parse_tree_function() {
        let input = vec![
            Token::new(TokenType::Func, 1),
            Token::new(TokenType::Identifier("test".to_string()), 1),
            Token::new(TokenType::Type(Types::I32), 1),
            Token::new(TokenType::Type(Types::I32), 1),
            Token::new(TokenType::Arrow, 1),
            Token::new(TokenType::Type(Types::I32), 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::Add, 1),
            Token::new(TokenType::RightBrace, 1),
            Token::new(TokenType::I32(0), 1),
            Token::new(TokenType::I32(1), 1),
            Token::new(TokenType::Identifier("test".to_string()), 1),
            Token::new(TokenType::Print, 1),
        ];

        let function_output = HashMap::from([(
            "test".to_string(),
            ParseTree::FuncDecl(
                "test".to_string(),
                vec![Types::I32, Types::I32],
                vec![Types::I32],
                Box::new(ParseTree::Region(vec![ParseTree::Element(Token::new(
                    TokenType::Add,
                    1,
                ))])),
            ),
        )]);

        let output = ParseTree::Region(vec![
            ParseTree::Element(Token::new(TokenType::I32(0), 1)),
            ParseTree::Element(Token::new(TokenType::I32(1), 1)),
            ParseTree::Element(Token::new(TokenType::Identifier("test".to_string()), 1)),
            ParseTree::Element(Token::new(TokenType::Print, 1)),
        ]);

        test_function(input, function_output, output);
    }
}
