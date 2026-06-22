use clap::Parser;

use crate::error::types::ParserError;
use crate::lexer::tokens::{Token, TokenType};
use crate::parser::parse_tree::{FuncDecl, ParseTree};
use crate::parser::stack_types::StackType;
use crate::parser::stack_values::StackValue;

use std::collections::HashMap;
use std::collections::HashSet;

pub struct Typing {}

impl Typing {
    pub fn type_check(
        tree: &ParseTree,
        functions: &HashMap<String, FuncDecl>,
    ) -> Result<(), ParserError> {
        let mut stack = Vec::new();
        let variable_lookup = HashMap::new();
        Self::type_check_stack(tree, &mut stack, functions, &variable_lookup)?;

        let variable_lookup = HashMap::new();
        for (_, f) in functions.iter() {
            Self::type_check_function(f, functions, &variable_lookup)?;
        }

        Ok(())
    }

    fn type_check_stack(
        tree: &ParseTree,
        stack: &mut Vec<StackType>,
        functions: &HashMap<String, FuncDecl>,
        variable_lookup: &HashMap<String, StackType>,
    ) -> Result<(), ParserError> {
        match tree {
            ParseTree::None => unreachable!("Invalid stack"),
            ParseTree::Element(t) => Self::type_check_token(t, stack, functions, variable_lookup)?,
            ParseTree::Region(r) => {
                for tree in r {
                    Self::type_check_stack(tree, stack, functions, variable_lookup)?;
                }
            }
            ParseTree::If(conds, (t_else, else_region)) => {
                let mut stacks = Vec::new();
                let mut stack_cond = stack.clone();
                for (t, c, r) in conds {
                    let mut stack_copy = stack_cond.clone();
                    Self::type_check_stack(c, &mut stack_copy, functions, variable_lookup)?;
                    Self::check_stack_length(&t, &stack_copy, 1)?;
                    Self::check_stack_types(&t, &stack_copy, &vec![StackType::Bool])?;
                    stack_copy.pop();
                    stack_cond = stack_copy.clone();

                    let mut stack_copy2 = stack_copy.clone();

                    Self::type_check_stack(r, &mut stack_copy2, functions, variable_lookup)?;
                    stacks.push((t, stack_copy, stack_copy2));
                }

                if stacks.len() > 1 {
                    for i in stacks.windows(2) {
                        let (t1, c1, r1) = &i[0];
                        let (_, c2, r2) = &i[1];

                        Self::verify_stack_equivalence(t1, c1, c2)?;
                        Self::verify_stack_equivalence(t1, r1, r2)?;
                    }
                }

                let (_, mut stack1, stack2) = stacks[0].clone();
                Self::type_check_stack(else_region, &mut stack1, functions, variable_lookup)?;
                Self::verify_stack_equivalence(&t_else, &stack1, &stack2)?;

                *stack = stack2;
            }
            ParseTree::While(t, cond, region) => {
                let mut stack_copy = stack.clone();
                Self::type_check_stack(cond, &mut stack_copy, functions, variable_lookup)?;
                Self::check_stack_length(&t, &stack_copy, 1)?;
                Self::check_stack_types(&t, &stack_copy, &vec![StackType::Bool])?;
                stack_copy.pop();

                Self::type_check_stack(region, &mut stack_copy, functions, variable_lookup)?;

                Self::verify_stack_equivalence(&t, stack, &stack_copy)?;
            }
            ParseTree::Assign(t, v, r) => {
                Self::check_stack_length(t, stack, v.len())?;
                let mut stack_copy: Vec<StackType> = stack[(stack.len() - v.len())..(stack.len())]
                    .iter()
                    .cloned()
                    .collect();

                let mut new_variable_lookup: HashMap<String, StackType> = variable_lookup.clone();

                for (v, t) in v.iter().zip(stack_copy.iter()) {
                    new_variable_lookup.insert(v.to_string(), t.clone());
                }

                Self::type_check_stack(r, &mut stack_copy, functions, &new_variable_lookup)?;
            }
            ParseTree::FuncDecl(func) => {
                Self::type_check_function(func, functions, variable_lookup)?;
            }
        };
        Ok(())
    }

    fn type_check_function(
        func: &FuncDecl,
        functions: &HashMap<String, FuncDecl>,
        variable_lookup: &HashMap<String, StackType>,
    ) -> Result<(), ParserError> {
        let mut stack = func.inputs.clone();
        Self::type_check_stack(&func.region, &mut stack, functions, variable_lookup)?;
        Self::check_stack_length(&func.token, &stack, func.outputs.len())?;
        Self::check_stack_types(
            &func.token,
            &stack,
            &func
                .outputs
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<StackType>>(),
        )?;

        Ok(())
    }

    fn verify_stack_equivalence(
        token: &Token,
        stack: &Vec<StackType>,
        stack_2: &Vec<StackType>,
    ) -> Result<(), ParserError> {
        if stack.len() != stack_2.len() {
            return Err(ParserError::InvalidShape(token.position_info.clone()));
        }

        for (t1, t2) in stack.iter().zip(stack_2.iter()) {
            if t1 != t2 {
                return Err(ParserError::InvalidShape(token.position_info.clone()));
            }
        }

        Ok(())
    }

    fn check_stack_length(
        token: &Token,
        stack: &Vec<StackType>,
        required_length: usize,
    ) -> Result<(), ParserError> {
        if stack.len() < required_length {
            return Err(ParserError::InvalidNumberOfArguments(
                token.position_info.clone(),
                required_length,
                stack.len(),
            ));
        } else {
            Ok(())
        }
    }

    fn check_stack_types(
        token: &Token,
        stack: &Vec<StackType>,
        required_types: &Vec<StackType>,
    ) -> Result<(), ParserError> {
        for (i, t) in (0..).zip(required_types.iter()) {
            let index = stack.len() - 1 - i;
            if stack.get(index).unwrap() != t {
                return Err(ParserError::InvalidType(
                    token.position_info.clone(),
                    t.clone(),
                    stack.get(index).unwrap().clone(),
                ));
            }
        }

        return Ok(());
    }

    fn check_stack_types_multi(
        token: &Token,
        stack: &Vec<StackType>,
        required_types: &Vec<HashSet<StackType>>,
    ) -> Result<(), ParserError> {
        for (i, t) in (0..).zip(required_types.iter()) {
            let index = stack.len() - 1 - i;
            if !t.contains(stack.get(index).unwrap()) {
                return Err(ParserError::InvalidTypeSet(
                    token.position_info.clone(),
                    t.clone(),
                    stack.get(index).unwrap().clone(),
                ));
            }
        }

        return Ok(());
    }

    fn type_check_token(
        token: &Token,
        stack: &mut Vec<StackType>,
        functions: &HashMap<String, FuncDecl>,
        variable_lookup: &HashMap<String, StackType>,
    ) -> Result<(), ParserError> {
        match &token.token_type {
            TokenType::LeftBrace => {}
            TokenType::RightBrace => {}
            TokenType::Add => {
                Self::check_stack_length(token, stack, 2)?;
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();

                match v2 {
                    StackType::I32 => (),
                    _ => {
                        return Err(ParserError::InvalidType(
                            token.position_info.clone(),
                            StackType::I32,
                            v2,
                        ));
                    }
                }

                match v1 {
                    StackType::I32 => {
                        stack.push(StackType::I32);
                    }
                    StackType::Ptr(t) => {
                        stack.push(StackType::Ptr(t));
                    }
                    _ => {
                        return Err(ParserError::InvalidTypeSet(
                            token.position_info.clone(),
                            HashSet::from([
                                StackType::I32,
                                StackType::Ptr(Box::new(StackType::I32)),
                            ]),
                            v1,
                        ));
                    }
                }
            }
            TokenType::Subtract | TokenType::Multiply | TokenType::Divide | TokenType::Modulo => {
                Self::check_stack_length(token, stack, 2)?;
                Self::check_stack_types(token, stack, &vec![StackType::I32, StackType::I32])?;
                stack.pop();
                stack.pop();
                stack.push(StackType::I32);
            }
            TokenType::Rotate3 => {
                Self::check_stack_length(token, stack, 3)?;
                let v3 = stack.pop().unwrap();
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();
                stack.push(v3);
                stack.push(v1);
                stack.push(v2);
            }
            TokenType::Duplicate => {
                Self::check_stack_length(token, stack, 1)?;
                stack.push(stack.last().unwrap().clone());
            }
            TokenType::Drop => {
                Self::check_stack_length(token, stack, 1)?;
                stack.pop().unwrap();
            }
            TokenType::Over => {
                Self::check_stack_length(token, stack, 2)?;
                stack.push(stack.get(stack.len() - 2).unwrap().clone());
            }
            TokenType::Swap => {
                Self::check_stack_length(token, stack, 2)?;
                let v1 = stack.pop().unwrap();
                let v2 = stack.pop().unwrap();
                stack.push(v1);
                stack.push(v2);
            }
            TokenType::Print => {
                Self::check_stack_length(token, stack, 1)?;
                stack.pop();
            }
            TokenType::Less
            | TokenType::Greater
            | TokenType::LessEqual
            | TokenType::GreaterEqual
            | TokenType::Equal
            | TokenType::NotEqual => {
                Self::check_stack_length(token, stack, 2)?;
                Self::check_stack_types(token, stack, &vec![StackType::I32, StackType::I32])?;
                stack.pop();
                stack.pop();
                stack.push(StackType::Bool);
            }
            TokenType::And | TokenType::Or => {
                Self::check_stack_length(token, stack, 2)?;
                Self::check_stack_types(token, stack, &vec![StackType::Bool, StackType::Bool])?;
                stack.pop();
                stack.pop();
                stack.push(StackType::Bool);
            }
            TokenType::Not => {
                Self::check_stack_length(token, stack, 1)?;
                Self::check_stack_types(token, stack, &vec![StackType::Bool])?;
                stack.pop();
                stack.push(StackType::Bool);
            }
            TokenType::Type(t) => stack.push(StackType::convert_type(&t)),
            TokenType::StringValue(_) => stack.push(StackType::String),
            TokenType::I32(_) => stack.push(StackType::I32),
            TokenType::BoolValue(_) => stack.push(StackType::Bool),

            TokenType::Identifier(s) => {
                if let Some(func) = functions.get(s) {
                    Self::check_stack_length(&token, &stack, func.inputs.len())?;
                    Self::check_stack_types(
                        &token,
                        &stack,
                        &func.inputs.iter().rev().cloned().collect(),
                    )?;

                    for _ in 0..func.inputs.len() {
                        stack.pop();
                    }

                    for o in func.outputs.iter() {
                        stack.push(o.clone());
                    }
                } else if let Some(t) = variable_lookup.get(s) {
                    stack.push(StackType::Var(Box::new(t.clone())));
                }
            }

            TokenType::Read => {
                Self::check_stack_length(&token, &stack, 1)?;
                let stack_value = stack.pop().unwrap();
                let stack_type = match stack_value {
                    StackType::Var(t) => *t,
                    StackType::Ptr(t) => *t,
                    _ => {
                        return Err(ParserError::InvalidTypeSet(
                            token.position_info.clone(),
                            HashSet::from([
                                StackType::Var(Box::new(StackType::String)),
                                StackType::Ptr(Box::new(StackType::I32)),
                            ]),
                            stack_value,
                        ));
                    }
                };
                stack.push(stack_type);
            }
            TokenType::Assign => {
                Self::check_stack_length(&token, &stack, 2)?;
                let write_value = stack.pop().unwrap();
                let stack_value = stack.pop().unwrap();
                let stack_type = match stack_value {
                    StackType::Var(t) => *t,
                    StackType::Ptr(t) => *t,
                    _ => {
                        return Err(ParserError::InvalidTypeSet(
                            token.position_info.clone(),
                            HashSet::from([
                                StackType::Var(Box::new(StackType::String)),
                                StackType::Ptr(Box::new(StackType::I32)),
                            ]),
                            stack_value,
                        ));
                    }
                };
                if write_value != stack_type {
                    return Err(ParserError::InvalidType(
                        token.position_info.clone(),
                        stack_type,
                        write_value,
                    ));
                }
            }
            TokenType::Mem => {
                Self::check_stack_length(&token, &stack, 2)?;

                if let Some(t) = stack.pop() {
                    match t {
                        StackType::I32 => (),
                        _ => {
                            return Err(ParserError::InvalidType(
                                token.position_info.clone(),
                                StackType::I32,
                                t,
                            ));
                        }
                    }
                }

                let t = stack.pop().unwrap();
                stack.push(StackType::Ptr(Box::new(t)));
            }
            TokenType::DebugPrintStack => {}

            TokenType::If
            | TokenType::Else
            | TokenType::While
            | TokenType::Func
            | TokenType::Arrow
            | TokenType::Assignment => {
                unreachable!();
            }
        }

        Ok(())
    }
}
