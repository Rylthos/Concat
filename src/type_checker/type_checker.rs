use crate::{
    ast::{
        raw_node::{AstNode, FuncDeclNode, Literal, RecordDeclNode},
        reduced_node::{ReducedAstNode, ReducedFuncDeclNode, ReducedRegion},
        typed_node::{
            TypedAssignNode, TypedAstNode, TypedFuncDeclNode, TypedIfNode, TypedRecordDeclNode,
            TypedRegion, TypedVariableNode, TypedWhileNode,
        },
    },
    builtins::{
        basic_types::BasicType,
        builtins::Builtin,
        reduced_builtins::ReducedBuiltin,
        typed_builtins::TypedBuiltin,
        types::{PtrType, Type, UnionType},
    },
    config::config::Config,
    error::type_error::TypeError,
};

use std::collections::HashMap;

pub struct TypeChecker {
    config: Config,
    reduced_tree: ReducedRegion,

    functions: HashMap<String, TypedFuncDeclNode>,
    pub(crate) records: HashMap<String, TypedRecordDeclNode>,
}

pub struct TypedData {
    pub main_region: TypedRegion,

    pub functions: HashMap<String, TypedFuncDeclNode>,
    pub records: HashMap<String, TypedRecordDeclNode>,
}

pub(crate) enum TaggedType {
    Type(Type),
    Ref(usize),
}

impl TypeChecker {
    pub fn init(config: Config, reduced_tree: ReducedRegion) -> TypeChecker {
        TypeChecker {
            config,
            reduced_tree,

            functions: HashMap::new(),
            records: HashMap::new(),
        }
    }

    pub fn type_check(&mut self) -> Result<TypedData, TypeError> {
        let parsed_region =
            self.type_check_region(self.reduced_tree.clone(), &mut Vec::new(), &HashMap::new())?;
        Ok(TypedData {
            main_region: parsed_region,
            functions: self.functions.clone(),
            records: self.records.clone(),
        })
    }

    pub(crate) fn stack_size(stack: &Vec<Type>, len: usize) -> Result<(), TypeError> {
        if stack.len() < len {
            todo!()
        }
        Ok(())
    }

    fn stack_shape(stack: &Vec<Type>, target: &[Type]) -> Result<(), TypeError> {
        let target_stack = Vec::from(target);
        Self::stack_size(stack, target_stack.len())?;

        for (i, t) in (0..).zip(target_stack.iter().rev()) {
            let s = stack[stack.len() - 1 - i].clone();
            if !t.can_become(&s) {
                todo!("{}: {}", t, s);
            }
        }

        Ok(())
    }

    pub(crate) fn stack_operation(
        stack: &mut Vec<Type>,
        inputs: &[TaggedType],
        outputs: &[TaggedType],
    ) -> Result<(), TypeError> {
        Self::stack_size(stack, inputs.len())?;

        let mut types = Vec::new();

        for _ in 0..(inputs.len()) {
            types.push(stack.pop().unwrap());
        }
        types = types.iter().rev().cloned().collect();

        let matches: Vec<Type> = inputs
            .iter()
            .map(|t| match t {
                TaggedType::Ref(index) => types.get(*index).unwrap(),
                TaggedType::Type(t) => t,
            })
            .cloned()
            .collect();

        if !types
            .iter()
            .zip(matches.iter())
            .all(|(a, b)| a.can_become(&b))
        {
            return Err(todo!("Convert {:?} -> {:?}", types, matches));
        };

        for o in outputs.iter() {
            match o {
                TaggedType::Ref(index) => stack.push(types.get(*index).unwrap().clone()),
                TaggedType::Type(t) => stack.push(t.clone()),
            }
        }

        Ok(())
    }

    pub(crate) fn compare_stacks(stack1: &Vec<Type>, stack2: &Vec<Type>) -> Result<(), TypeError> {
        if stack1.len() != stack2.len() {
            todo!()
        }

        for (a, b) in stack1.iter().zip(stack2.iter()) {
            if !b.can_become(a) {
                todo!()
            }
        }

        Ok(())
    }

    fn type_check_function(
        &mut self,
        function: &ReducedFuncDeclNode,
    ) -> Result<TypedFuncDeclNode, TypeError> {
        let mut input_stack = function
            .inputs
            .iter()
            .map(|t| Type::from_basic_type(&t))
            .collect();

        let typed_region =
            self.type_check_region(function.region.clone(), &mut input_stack, &HashMap::new())?;

        let outputs: Vec<Type> = function
            .outputs
            .iter()
            .map(|t| Type::from_basic_type(&t))
            .collect();

        match Self::stack_shape(&input_stack, &outputs) {
            Ok(_) => (),
            Err(_) => {
                todo!();
            }
        }

        Ok(TypedFuncDeclNode {
            position: function.position.clone(),
            name: function.name.clone(),
            inputs: function
                .inputs
                .iter()
                .map(|t| Type::from_basic_type(&t))
                .collect(),
            outputs: function
                .outputs
                .iter()
                .map(|t| Type::from_basic_type(&t))
                .collect(),
            region: typed_region,
        })
    }

    pub(crate) fn type_check_region(
        &mut self,
        region: ReducedRegion,
        stack: &mut Vec<Type>,
        lookup: &HashMap<String, (Type, usize, usize)>,
    ) -> Result<TypedRegion, TypeError> {
        let mut nodes = Vec::new();

        for node in region.region.iter() {
            if let Some(typed_node) = self.type_check_node(node, stack, lookup)? {
                nodes.push(typed_node);
            }
        }

        Ok(TypedRegion { region: nodes })
    }

    fn type_check_node(
        &mut self,
        node: &ReducedAstNode,
        stack: &mut Vec<Type>,
        lookup: &HashMap<String, (Type, usize, usize)>,
    ) -> Result<Option<TypedAstNode>, TypeError> {
        match node {
            ReducedAstNode::Builtin(p, b) => Ok(Some(TypedAstNode::Builtin(
                p.clone(),
                self.type_check_builtin(&b.clone(), stack)?,
            ))),
            ReducedAstNode::Literal(literal) => {
                if self.functions.contains_key(&literal.literal) {
                    let function = self.functions.get(&literal.literal).unwrap();
                    for _ in 0..function.inputs.len() {
                        stack.pop();
                    }
                    stack.append(&mut function.outputs.clone());
                    Ok(Some(TypedAstNode::FunctionCall(literal.clone())))
                } else if lookup.contains_key(&literal.literal) {
                    let variable = lookup.get(&literal.literal).unwrap();
                    stack.push(Type::Var(Box::new(variable.0.clone())));
                    Ok(Some(TypedAstNode::Variable(TypedVariableNode {
                        position: literal.position.clone(),
                        name: literal.literal.clone(),
                        depth: variable.1,
                        offset: variable.2,
                        r#type: lookup.get(&literal.literal).unwrap().clone().0,
                    })))
                } else {
                    todo!();
                }
            }
            ReducedAstNode::RecordElementIdentifier(literal) => {
                self.type_check_record_identifier(literal, stack)
            }
            ReducedAstNode::WriteRecordElementIdentifier(literal) => {
                self.type_check_write_record_identifier(literal, stack)
            }
            ReducedAstNode::Assign(assign_node) => {
                self.type_check_assign(assign_node, stack, lookup)
            }
            ReducedAstNode::If(if_node) => self.type_check_if(if_node, stack, lookup),
            ReducedAstNode::While(while_node) => self.type_check_while(while_node, stack, lookup),
            ReducedAstNode::FuncDecl(func_decl) => {
                let func = self.type_check_function(func_decl)?;
                self.functions.insert(func_decl.name.clone(), func);
                Ok(None)
            }
            ReducedAstNode::RecordDecl(record_decl) => {
                self.records.insert(
                    record_decl.name.clone(),
                    TypedRecordDeclNode {
                        position: record_decl.position.clone(),
                        name: record_decl.name.clone(),
                        entries: record_decl
                            .entries
                            .iter()
                            .map(|(n, t)| (n.clone(), Type::from_basic_type(&t)))
                            .collect(),
                    },
                );
                Ok(None)
            }
        }
    }
}
