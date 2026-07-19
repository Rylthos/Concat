use clap::error::Result;

use crate::ast::raw_node::{AstNode, Literal, Region};
use crate::ast::reduced_node::*;
use crate::builtins::basic_stack_values::BasicStackValue;
use crate::builtins::builtins::Builtin;
use crate::builtins::reduced_builtins::ReducedBuiltin;
use crate::config::config::Config;
use crate::error::reducer_error::ReducerError;
use crate::lexer::tokens::PositionInfo;

use std::collections::HashMap;

pub struct Reducer {
    config: Config,
    raw_tree: Region,

    defines: HashMap<String, ReducedBuiltin>,
}

impl Reducer {
    pub fn init(config: Config, tree: Region) -> Reducer {
        Reducer {
            config,
            raw_tree: tree,
            defines: HashMap::new(),
        }
    }

    pub fn reduce(&mut self) -> Result<ReducedRegion, ReducerError> {
        self.reduce_region(&self.raw_tree.clone())
    }

    pub fn reduce_region(&mut self, region: &Region) -> Result<ReducedRegion, ReducerError> {
        let mut reduced_nodes = Vec::new();
        for node in region.region.iter() {
            if let Some(reduced_node) = self.reduce_node(node, &mut reduced_nodes)? {
                reduced_nodes.push(reduced_node);
            }
        }

        Ok(ReducedRegion {
            region: reduced_nodes,
        })
    }

    pub fn reduce_node(
        &mut self,
        node: &AstNode,
        current_region: &mut Vec<ReducedAstNode>,
    ) -> Result<Option<ReducedAstNode>, ReducerError> {
        match node {
            AstNode::Builtin(pos, builtin) => {
                if let Some(reduced_builtin) = self.reduce_builtin(pos, builtin, current_region)? {
                    Ok(Some(ReducedAstNode::Builtin(pos.clone(), reduced_builtin)))
                } else {
                    Ok(None)
                }
            }

            AstNode::Literal(literal) => self.check_literal(literal),
            AstNode::RecordElementIdentifier(literal) => Ok(Some(
                ReducedAstNode::RecordElementIdentifier(literal.clone()),
            )),
            AstNode::WriteRecordElementIdentifier(literal) => Ok(Some(
                ReducedAstNode::WriteRecordElementIdentifier(literal.clone()),
            )),
            AstNode::Assign(assign_node) => Ok(Some(ReducedAstNode::Assign(ReducedAssignNode {
                position: assign_node.position.clone(),
                labels: assign_node.labels.clone(),
                region: self.reduce_region(&assign_node.region)?,
            }))),
            AstNode::If(if_node) => {
                let mut reduced_if_region = Vec::new();
                for (c, r) in if_node.if_region.iter() {
                    reduced_if_region.push((self.reduce_region(&c)?, self.reduce_region(&r)?));
                }

                Ok(Some(ReducedAstNode::If(ReducedIfNode {
                    position: if_node.position.clone(),
                    if_region: reduced_if_region,
                    else_region: self.reduce_region(&if_node.else_region)?,
                })))
            }
            AstNode::While(while_node) => Ok(Some(ReducedAstNode::While(ReducedWhileNode {
                position: while_node.position.clone(),
                condition: self.reduce_region(&while_node.condition)?,
                region: self.reduce_region(&while_node.region)?,
            }))),
            AstNode::FuncDecl(func_decl_node) => {
                Ok(Some(ReducedAstNode::FuncDecl(ReducedFuncDeclNode {
                    position: func_decl_node.position.clone(),
                    name: func_decl_node.name.clone(),
                    inputs: func_decl_node.inputs.clone(),
                    outputs: func_decl_node.outputs.clone(),
                    region: self.reduce_region(&func_decl_node.region)?,
                })))
            }
            AstNode::RecordDecl(record_decl_node) => {
                Ok(Some(ReducedAstNode::RecordDecl(record_decl_node.clone())))
            }
        }
    }

    pub fn check_literal(&self, literal: &Literal) -> Result<Option<ReducedAstNode>, ReducerError> {
        if self.defines.contains_key(&literal.literal) {
            Ok(Some(ReducedAstNode::Builtin(
                literal.position.clone(),
                self.defines.get(&literal.literal).unwrap().clone(),
            )))
        } else {
            Ok(Some(ReducedAstNode::Literal(literal.clone())))
        }
    }

    pub fn reduce_builtin(
        &mut self,
        pos: &PositionInfo,
        builtin: &Builtin,
        current_region: &mut Vec<ReducedAstNode>,
    ) -> Result<Option<ReducedBuiltin>, ReducerError> {
        match builtin {
            Builtin::RawNth | Builtin::RawNthWrite | Builtin::RawUnion => {
                if let Some(ReducedAstNode::Builtin(_, ReducedBuiltin::I32Value(i))) =
                    current_region.pop()
                {
                    match builtin {
                        Builtin::RawNth => Ok(Some(ReducedBuiltin::Nth(i as usize))),
                        Builtin::RawNthWrite => Ok(Some(ReducedBuiltin::NthWrite(i as usize))),
                        Builtin::RawUnion => Ok(Some(ReducedBuiltin::Union(i as usize))),
                        _ => unreachable!(),
                    }
                } else {
                    Err(ReducerError::ExpectedIntConstant(pos.clone()))
                }
            }

            Builtin::Define => {
                if let Some(ReducedAstNode::Literal(l)) = current_region.pop()
                    && let Some(ReducedAstNode::Builtin(_, value)) = current_region.pop()
                {
                    self.defines.insert(l.literal.clone(), value);
                    Ok(None)
                } else {
                    Err(ReducerError::InvalidDefine(pos.clone()))
                }
            }

            _ => Ok(Some(self.reduce_simplebuiltin(builtin))),
        }
    }

    pub fn reduce_simplebuiltin(&mut self, builtin: &Builtin) -> ReducedBuiltin {
        match builtin {
            Builtin::Add => ReducedBuiltin::Add,
            Builtin::Divide => ReducedBuiltin::Divide,
            Builtin::Modulo => ReducedBuiltin::Modulo,
            Builtin::Multiply => ReducedBuiltin::Multiply,
            Builtin::Subtract => ReducedBuiltin::Subtract,
            Builtin::Drop => ReducedBuiltin::Drop,
            Builtin::Duplicate => ReducedBuiltin::Duplicate,
            Builtin::Over => ReducedBuiltin::Over,
            Builtin::Swap => ReducedBuiltin::Swap,
            Builtin::Rotate3 => ReducedBuiltin::Rotate3,
            Builtin::Print => ReducedBuiltin::Print,
            Builtin::Less => ReducedBuiltin::Less,
            Builtin::Greater => ReducedBuiltin::Greater,
            Builtin::LessEqual => ReducedBuiltin::LessEqual,
            Builtin::GreaterEqual => ReducedBuiltin::GreaterEqual,
            Builtin::Equal => ReducedBuiltin::Equal,
            Builtin::NotEqual => ReducedBuiltin::NotEqual,
            Builtin::And => ReducedBuiltin::And,
            Builtin::Or => ReducedBuiltin::Or,
            Builtin::Assign => ReducedBuiltin::Assign,
            Builtin::Read => ReducedBuiltin::Read,

            Builtin::Input => ReducedBuiltin::Input,

            Builtin::Mem => ReducedBuiltin::Mem,

            Builtin::Record(name) => ReducedBuiltin::Record(name.to_string()),
            Builtin::RecordType(iden) => ReducedBuiltin::RecordType(iden.to_string()),

            Builtin::BasicType(t) => ReducedBuiltin::BasicType(t.clone()),
            Builtin::StringValue(s) => ReducedBuiltin::StringValue(s.to_string()),
            Builtin::I32Value(i) => ReducedBuiltin::I32Value(*i),
            Builtin::BoolValue(b) => ReducedBuiltin::BoolValue(*b),
            Builtin::CharValue(c) => ReducedBuiltin::CharValue(*c),
            Builtin::DebugPrintStack => ReducedBuiltin::DebugPrintStack,
            Builtin::DebugHeapStack => ReducedBuiltin::DebugHeapStack,

            _ => unreachable!(),
        }
    }
}
