use crate::lexer::tokens::{Token, TokenType};
use crate::parser::instructions::{Instruction, StackValue};

fn parse_region<'a>(
    mut tokens: impl Iterator<Item = &'a Token>,
) -> Result<Vec<Instruction>, String> {
    let mut parsed_expression: Vec<Instruction> = Vec::new();

    let mut peekable_tokens = tokens.peekable();

    while let Some(token) = peekable_tokens.peek() {
        match &token.token_type {
            TokenType::StringValue(s) => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Push(StackValue::String(s.to_string())))
            }
            TokenType::NumberValue(n) => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Push(StackValue::Number(*n)))
            }
            TokenType::BoolValue(b) => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Push(StackValue::Bool(*b)))
            }
            TokenType::Type(t) => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Push(StackValue::Type(t.clone())))
            }
            //
            TokenType::Add => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Add)
            }
            TokenType::Subtract => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Subtract)
            }
            TokenType::Divide => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Divide)
            }
            TokenType::Multiply => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Multiply)
            }
            //
            TokenType::Rotate => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Rotate)
            }
            TokenType::Duplicate => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Duplicate)
            }
            TokenType::Drop => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Drop)
            }
            TokenType::Over => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Over)
            }
            TokenType::Swap => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Swap)
            }
            TokenType::Cast => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Cast)
            }
            TokenType::Print => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Print)
            }
            //
            TokenType::Less => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Less)
            }
            TokenType::LessEqual => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::LessEqual)
            }
            TokenType::Greater => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Greater)
            }
            TokenType::GreaterEqual => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::GreaterEqual)
            }
            TokenType::Equal => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::Equal)
            }
            TokenType::NotEqual => {
                peekable_tokens.next();
                parsed_expression.push(Instruction::NotEqual)
            }
            //
            TokenType::While => {
                // Search for 'block'
                // Insert Goto within block the iterator, r
                todo!("While")
            }
            //
            _ => panic!("Unhandled Token: {:?}", token.token_type),
        }
    }

    return Ok(parsed_expression);
}

fn parse(tokens: &Vec<Token>) -> Result<Vec<Instruction>, String> {
    let mut iter = tokens.iter();

    let parsed_expression = parse_region(&mut iter);
    return parsed_expression;
}

pub fn parse_tokens(tokens: &Vec<Token>) -> Vec<Instruction> {
    let expr_result = parse(tokens);

    match expr_result {
        Ok(expr) => return expr,
        Err(s) => panic!("Parsing failed: {}", s),
    }
}
