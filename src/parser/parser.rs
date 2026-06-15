use crate::lexer::tokens::{Token, TokenType};
use crate::parser::instructions::{Instruction, StackValue};

#[derive(Debug, Clone)]
enum ParseTree {
    Element(Token),
    Region(Vec<ParseTree>),
    If(Vec<(Box<ParseTree>, Box<ParseTree>)>, Box<ParseTree>),
    While(Box<ParseTree>, Box<ParseTree>),
}

fn parse_element(token: Token) -> Option<Instruction> {
    let instr = match token.token_type {
        TokenType::StringValue(s) => Instruction::Push(StackValue::String(s.to_string())),
        TokenType::NumberValue(n) => Instruction::Push(StackValue::Number(n)),
        TokenType::BoolValue(b) => Instruction::Push(StackValue::Bool(b)),
        TokenType::Type(t) => Instruction::Push(StackValue::Type(t.clone())),
        //
        TokenType::Add => Instruction::Add,
        TokenType::Subtract => Instruction::Subtract,
        TokenType::Divide => Instruction::Divide,
        TokenType::Multiply => Instruction::Multiply,
        //
        TokenType::Rotate => Instruction::Rotate,
        TokenType::Duplicate => Instruction::Duplicate,
        TokenType::Drop => Instruction::Drop,
        TokenType::Over => Instruction::Over,
        TokenType::Swap => Instruction::Swap,
        TokenType::Cast => Instruction::Cast,
        TokenType::Print => Instruction::Print,
        //
        TokenType::Less => Instruction::Less,
        TokenType::LessEqual => Instruction::LessEqual,
        TokenType::Greater => Instruction::Greater,
        TokenType::GreaterEqual => Instruction::GreaterEqual,
        TokenType::Equal => Instruction::Equal,
        TokenType::NotEqual => Instruction::NotEqual,
        //
        _ => todo!("Unhandled: {:?}", token),
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
    }

    return Ok(parsed_expression);
}

fn get_condition<'a>(tokens: impl Iterator<Item = &'a Token>) -> Result<Vec<Token>, String> {
    let mut values: Vec<Token> = Vec::new();

    let mut peekable = tokens.peekable();

    while let Some(&t) = peekable.peek() {
        match t.token_type {
            TokenType::LeftBrace => {
                break;
            }
            _ => {
                peekable.next();
                values.push(t.clone());
            }
        }
    }

    return Ok(values);
}

fn get_region<'a>(tokens: impl Iterator<Item = &'a Token>) -> Result<Vec<Token>, String> {
    let mut values: Vec<Token> = Vec::new();

    let mut peekable = tokens.peekable();

    let mut count = 0;

    while let Some(&t) = peekable.peek() {
        match t.token_type {
            TokenType::LeftBrace => {
                if count == 0 {
                    peekable.next();
                    continue;
                }
                count += 1;
                peekable.next();
            }
            TokenType::RightBrace => {
                if count == 0 {
                    break;
                }
                count -= 1;
            }
            _ => {}
        }
        values.push(t.clone());
        peekable.next();
    }

    return Ok(values);
}

fn generate_parse_tree<'a>(
    mut tokens: impl Iterator<Item = &'a Token>,
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
                        generate_parse_tree(get_condition(&mut peekable)?.iter())?;
                    let region_tree = generate_parse_tree(get_region(&mut peekable)?.iter())?;

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

                                else_region =
                                    generate_parse_tree(get_region(&mut peekable)?.iter())?;
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
                let conditional_tree = generate_parse_tree(get_condition(&mut peekable)?.iter())?;
                let region_tree = generate_parse_tree(get_region(&mut peekable)?.iter())?;

                region.push(ParseTree::While(
                    Box::new(conditional_tree),
                    Box::new(region_tree),
                ))
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
    let result = generate_parse_tree(tokens.iter());

    let tree = match result {
        Ok(t) => t,
        Err(e) => panic!("Invalid: {}", e),
    };

    let expression = parse_tree(tree);

    // println!("Expression: {:?}", expression);

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

    #[test]
    fn parse_tree_normal() {
        let input = vec![
            Token::new(TokenType::Add, 1),
            Token::new(TokenType::Subtract, 1),
            Token::new(TokenType::Multiply, 1),
        ];
        let tree = generate_parse_tree(input.iter());

        match tree {
            Ok(t) => {
                let output = format!("{:?}", t);
                assert_eq!(
                    format!(
                        "{:?}",
                        ParseTree::Region(vec![
                            ParseTree::Element(Token::new(TokenType::Add, 1)),
                            ParseTree::Element(Token::new(TokenType::Subtract, 1)),
                            ParseTree::Element(Token::new(TokenType::Multiply, 1)),
                        ])
                    ),
                    output
                )
            }
            Err(_) => assert!(false, "Error"),
        }
    }

    #[test]
    fn parse_tree_if() {
        let input = vec![
            Token::new(TokenType::NumberValue(0.0), 1),
            Token::new(TokenType::If, 1),
            Token::new(TokenType::NumberValue(10.0), 1),
            Token::new(TokenType::Greater, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::RightBrace, 1),
        ];
        let tree = generate_parse_tree(input.iter());

        match tree {
            Ok(t) => {
                let output = format!("{:?}", t);
                assert_eq!(
                    format!(
                        "{:?}",
                        ParseTree::Region(vec![
                            ParseTree::Element(Token::new(TokenType::NumberValue(0.0), 1)),
                            ParseTree::If(
                                vec![(
                                    Box::new(ParseTree::Region(vec![
                                        ParseTree::Element(Token::new(
                                            TokenType::NumberValue(10.0),
                                            1
                                        )),
                                        ParseTree::Element(Token::new(TokenType::Greater, 1)),
                                    ])),
                                    Box::new(ParseTree::Region(vec![]))
                                ),],
                                Box::new(ParseTree::Region(vec![]))
                            ),
                        ])
                    ),
                    output
                )
            }
            Err(_) => assert!(false, "Error"),
        }
    }

    #[test]
    fn parse_tree_if_else() {
        let input = vec![
            Token::new(TokenType::NumberValue(0.0), 1),
            Token::new(TokenType::If, 1),
            Token::new(TokenType::NumberValue(10.0), 1),
            Token::new(TokenType::Greater, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::NumberValue(2.0), 1),
            Token::new(TokenType::RightBrace, 1),
            Token::new(TokenType::Else, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::NumberValue(3.0), 1),
            Token::new(TokenType::RightBrace, 1),
        ];
        let tree = generate_parse_tree(input.iter());

        match tree {
            Ok(t) => {
                let output = format!("{:?}", t);
                assert_eq!(
                    format!(
                        "{:?}",
                        ParseTree::Region(vec![
                            ParseTree::Element(Token::new(TokenType::NumberValue(0.0), 1)),
                            ParseTree::If(
                                vec![(
                                    Box::new(ParseTree::Region(vec![
                                        ParseTree::Element(Token::new(
                                            TokenType::NumberValue(10.0),
                                            1
                                        )),
                                        ParseTree::Element(Token::new(TokenType::Greater, 1)),
                                    ])),
                                    Box::new(ParseTree::Region(vec![ParseTree::Element(
                                        Token::new(TokenType::NumberValue(2.0), 1)
                                    )]))
                                ),],
                                Box::new(ParseTree::Region(vec![ParseTree::Element(Token::new(
                                    TokenType::NumberValue(3.0),
                                    1
                                ))]))
                            ),
                        ])
                    ),
                    output
                )
            }
            Err(_) => assert!(false, "Error"),
        }
    }

    #[test]
    fn parse_tree_if_elseif_else() {
        let input = vec![
            Token::new(TokenType::NumberValue(0.0), 1),
            Token::new(TokenType::If, 1),
            Token::new(TokenType::Duplicate, 1),
            Token::new(TokenType::NumberValue(10.0), 1),
            Token::new(TokenType::Greater, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::NumberValue(2.0), 1),
            Token::new(TokenType::RightBrace, 1),
            Token::new(TokenType::Else, 1),
            Token::new(TokenType::If, 1),
            Token::new(TokenType::Duplicate, 1),
            Token::new(TokenType::NumberValue(20.0), 1),
            Token::new(TokenType::Greater, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::NumberValue(3.0), 1),
            Token::new(TokenType::RightBrace, 1),
            Token::new(TokenType::Else, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::Drop, 1),
            Token::new(TokenType::NumberValue(4.0), 1),
            Token::new(TokenType::RightBrace, 1),
        ];
        let tree = generate_parse_tree(input.iter());

        match tree {
            Ok(t) => {
                let output = format!("{:?}", t);
                assert_eq!(
                    format!(
                        "{:?}",
                        ParseTree::Region(vec![
                            ParseTree::Element(Token::new(TokenType::NumberValue(0.0), 1)),
                            ParseTree::If(
                                vec![
                                    (
                                        Box::new(ParseTree::Region(vec![
                                            ParseTree::Element(Token::new(TokenType::Duplicate, 1)),
                                            ParseTree::Element(Token::new(
                                                TokenType::NumberValue(10.0),
                                                1
                                            )),
                                            ParseTree::Element(Token::new(TokenType::Greater, 1)),
                                        ])),
                                        Box::new(ParseTree::Region(vec![ParseTree::Element(
                                            Token::new(TokenType::NumberValue(2.0), 1)
                                        )]))
                                    ),
                                    (
                                        Box::new(ParseTree::Region(vec![
                                            ParseTree::Element(Token::new(TokenType::Duplicate, 1)),
                                            ParseTree::Element(Token::new(
                                                TokenType::NumberValue(20.0),
                                                1
                                            )),
                                            ParseTree::Element(Token::new(TokenType::Greater, 1)),
                                        ])),
                                        Box::new(ParseTree::Region(vec![ParseTree::Element(
                                            Token::new(TokenType::NumberValue(3.0), 1)
                                        )]))
                                    ),
                                ],
                                Box::new(ParseTree::Region(vec![
                                    ParseTree::Element(Token::new(TokenType::Drop, 1)),
                                    ParseTree::Element(Token::new(TokenType::NumberValue(4.0), 1))
                                ]))
                            ),
                        ])
                    ),
                    output
                )
            }
            Err(_) => assert!(false, "Error"),
        }
    }

    #[test]
    fn parse_tree_while() {
        let input = vec![
            Token::new(TokenType::NumberValue(0.0), 1),
            Token::new(TokenType::While, 1),
            Token::new(TokenType::Duplicate, 1),
            Token::new(TokenType::NumberValue(10.0), 1),
            Token::new(TokenType::Less, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::NumberValue(1.0), 1),
            Token::new(TokenType::Add, 1),
            Token::new(TokenType::RightBrace, 1),
        ];
        let tree = generate_parse_tree(input.iter());

        match tree {
            Ok(t) => {
                let output = format!("{:?}", t);
                assert_eq!(
                    format!(
                        "{:?}",
                        ParseTree::Region(vec![
                            ParseTree::Element(Token::new(TokenType::NumberValue(0.0), 1)),
                            ParseTree::While(
                                Box::new(ParseTree::Region(vec![
                                    ParseTree::Element(Token::new(TokenType::Duplicate, 1)),
                                    ParseTree::Element(Token::new(TokenType::NumberValue(10.0), 1)),
                                    ParseTree::Element(Token::new(TokenType::Less, 1))
                                ])),
                                Box::new(ParseTree::Region(vec![
                                    ParseTree::Element(Token::new(TokenType::NumberValue(1.0), 1)),
                                    ParseTree::Element(Token::new(TokenType::Add, 1)),
                                ]))
                            )
                        ])
                    ),
                    output
                )
            }
            Err(_) => assert!(false, "Error"),
        }
    }
}
