use crate::{
    ast::{
        raw_node::{AstNode, FuncDeclNode, Literal, RecordDeclNode},
        reduced_node::{ReducedAstNode, ReducedFuncDeclNode, ReducedRegion},
        typed_node::{
            TypedAssignNode, TypedAstNode, TypedFuncDeclNode, TypedIfNode, TypedRegion,
            TypedVariableNode, TypedWhileNode,
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
    records: HashMap<String, RecordDeclNode>,
}

pub struct TypedData {
    pub main_region: TypedRegion,

    pub functions: HashMap<String, TypedFuncDeclNode>,
}

enum TaggedType {
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
        })
    }

    fn stack_size(stack: &Vec<Type>, len: usize) -> Result<(), TypeError> {
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
                todo!();
            }
        }

        Ok(())
    }

    fn stack_operation(
        stack: &mut Vec<Type>,
        inputs: &[TaggedType],
        outputs: &[TaggedType],
    ) -> Result<(), TypeError> {
        Self::stack_size(stack, inputs.len())?;

        let mut types = Vec::new();

        for _ in 0..(inputs.len()) {
            types.push(stack.pop().unwrap());
        }

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
            return Err(todo!());
        };

        for o in outputs.iter() {
            match o {
                TaggedType::Ref(index) => stack.push(types.get(*index).unwrap().clone()),
                TaggedType::Type(t) => stack.push(t.clone()),
            }
        }

        Ok(())
    }

    fn compare_stacks(stack1: &Vec<Type>, stack2: &Vec<Type>) -> Result<(), TypeError> {
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
                todo!()
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

    fn type_check_region(
        &mut self,
        region: ReducedRegion,
        stack: &mut Vec<Type>,
        lookup: &HashMap<String, Type>,
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
        lookup: &HashMap<String, Type>,
    ) -> Result<Option<TypedAstNode>, TypeError> {
        match node {
            ReducedAstNode::Builtin(p, b) => Ok(Some(TypedAstNode::Builtin(
                p.clone(),
                self.type_check_builtin(b.clone(), stack)?,
            ))),
            ReducedAstNode::Literal(literal) => {
                if self.functions.contains_key(&literal.literal) {
                    Ok(Some(TypedAstNode::FunctionCall(literal.clone())))
                } else if lookup.contains_key(&literal.literal) {
                    Ok(Some(TypedAstNode::Variable(TypedVariableNode {
                        position: literal.position.clone(),
                        name: literal.literal.clone(),
                        depth: 0,
                        offset: 0,
                        r#type: lookup.get(&literal.literal).unwrap().clone(),
                    })))
                } else {
                    todo!();
                }
            }
            ReducedAstNode::RecordElementIdentifier(literal) => {
                Self::stack_size(&stack, 1)?;

                let record_name = if let stack_value = stack.pop().unwrap() {
                    match stack_value {
                        Type::RecordIden(s) => s,
                        _ => todo!(),
                    }
                } else {
                    todo!()
                };

                if let Some(record) = self.records.get(&record_name) {
                    let entries: Vec<_> = record
                        .entries
                        .iter()
                        .zip(0..)
                        .filter(|((name, _), _)| *name == literal.literal)
                        .collect();

                    if entries.len() == 0 {
                        todo!()
                    }

                    let ((_, stack_type), index) = entries[0];

                    stack.push(Type::from_basic_type(&stack_type));
                    Ok(Some(TypedAstNode::Builtin(
                        literal.position.clone(),
                        TypedBuiltin::Nth(index),
                    )))
                } else {
                    todo!()
                }
            }
            ReducedAstNode::WriteRecordElementIdentifier(literal) => {
                Self::stack_size(&stack, 2)?;

                let record_name = if let stack_value = stack.pop().unwrap() {
                    match stack_value {
                        Type::RecordIden(s) => s,
                        _ => todo!(),
                    }
                } else {
                    todo!()
                };

                let write_value = stack.pop().unwrap();

                if let Some(record) = self.records.get(&record_name) {
                    let entries: Vec<_> = record
                        .entries
                        .iter()
                        .zip(0..)
                        .filter(|((name, _), _)| *name == literal.literal)
                        .collect();

                    if entries.len() == 0 {
                        todo!()
                    }

                    let ((_, stack_type), index) = entries[0];

                    if !Type::from_basic_type(&stack_type).can_become(&write_value) {
                        todo!()
                    }

                    stack.push(Type::from_basic_type(&stack_type));
                    Ok(Some(TypedAstNode::Builtin(
                        literal.position.clone(),
                        TypedBuiltin::NthWrite(index),
                    )))
                } else {
                    todo!()
                }
            }
            ReducedAstNode::Assign(assign_node) => {
                Self::stack_size(stack, assign_node.labels.len())?;
                let stack_copy: Vec<Type> = stack
                    [(stack.len() - assign_node.labels.len())..(stack.len())]
                    .iter()
                    .cloned()
                    .collect();

                let mut variable_lookup = lookup.clone();

                for (v, t) in assign_node.labels.iter().zip(stack_copy.iter()) {
                    variable_lookup.insert(v.clone(), t.clone());
                }

                let mut new_stack = Vec::new();
                let region = self.type_check_region(
                    assign_node.region.clone(),
                    &mut new_stack,
                    &variable_lookup,
                )?;

                Ok(Some(TypedAstNode::Assign(TypedAssignNode {
                    position: assign_node.position.clone(),
                    labels: assign_node.labels.clone(),
                    region,
                })))
            }
            ReducedAstNode::If(if_node) => {
                let mut if_region = Vec::new();
                let mut stacks = Vec::new();
                let stack_cond = stack.clone();

                for (c, r) in if_node.if_region.clone() {
                    let mut stack_copy = stack_cond.clone();
                    let condition = self.type_check_region(c, &mut stack_copy, lookup)?;
                    Self::stack_operation(&mut stack_copy, &[TaggedType::Type(Type::Bool)], &[]);

                    let mut stack_copy2 = stack_copy.clone();

                    let region = self.type_check_region(r, &mut stack_copy2, lookup)?;
                    stacks.push((stack_copy, stack_copy2));
                    if_region.push((condition, region));
                }

                if stacks.len() > 1 {
                    for i in stacks.windows(2) {
                        let (c1, r1) = &i[0];
                        let (c2, r2) = &i[1];

                        Self::compare_stacks(c1, c2)?;
                        Self::compare_stacks(r1, r2)?;
                    }
                }

                let &mut (ref mut stack1, ref stack2) = stacks.get_mut(0).unwrap();
                let else_region =
                    self.type_check_region(if_node.else_region.clone(), stack1, lookup)?;
                Self::compare_stacks(&stack1, &stack2)?;

                *stack = stack2.to_vec();

                Ok(Some(TypedAstNode::If(TypedIfNode {
                    position: if_node.position.clone(),
                    if_region,
                    else_region,
                })))
            }
            ReducedAstNode::While(while_node) => {
                let mut stack_copy = stack.clone();
                let condition =
                    self.type_check_region(while_node.region.clone(), &mut stack_copy, lookup)?;
                Self::stack_operation(stack, &[TaggedType::Type(Type::Bool)], &[]);
                stack_copy.pop();

                let region =
                    self.type_check_region(while_node.region.clone(), &mut stack_copy, lookup)?;

                Self::compare_stacks(stack, &stack_copy)?;

                Ok(Some(TypedAstNode::While(TypedWhileNode {
                    position: while_node.position.clone(),
                    condition,
                    region,
                })))
            }
            ReducedAstNode::FuncDecl(func_decl) => {
                let func = self.type_check_function(func_decl)?;
                self.functions.insert(func_decl.name.clone(), func);
                Ok(None)
            }
            ReducedAstNode::RecordDecl(record_decl) => {
                self.records
                    .insert(record_decl.name.clone(), record_decl.clone());
                Ok(None)
            }
        }
    }

    fn type_check_builtin(
        &self,
        builtin: ReducedBuiltin,
        stack: &mut Vec<Type>,
    ) -> Result<TypedBuiltin, TypeError> {
        match builtin {
            ReducedBuiltin::Add => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::I32)],
                )?;

                Ok(TypedBuiltin::Add)
            }
            ReducedBuiltin::Divide => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::I32)],
                )?;

                Ok(TypedBuiltin::Divide)
            }
            ReducedBuiltin::Modulo => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::I32)],
                )?;

                Ok(TypedBuiltin::Modulo)
            }
            ReducedBuiltin::Multiply => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::I32)],
                )?;

                Ok(TypedBuiltin::Multiply)
            }
            ReducedBuiltin::Subtract => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::I32)],
                )?;

                Ok(TypedBuiltin::Subtract)
            }

            ReducedBuiltin::Drop => {
                Self::stack_size(stack, 1)?;
                stack.pop();

                Ok(TypedBuiltin::Drop)
            }
            ReducedBuiltin::Duplicate => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Ref(0)],
                    &[TaggedType::Ref(0), TaggedType::Ref(0)],
                )?;

                Ok(TypedBuiltin::Drop)
            }
            ReducedBuiltin::Over => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Ref(0), TaggedType::Ref(1)],
                    &[TaggedType::Ref(0), TaggedType::Ref(1), TaggedType::Ref(0)],
                )?;

                Ok(TypedBuiltin::Over)
            }
            ReducedBuiltin::Swap => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Ref(0), TaggedType::Ref(1)],
                    &[TaggedType::Ref(1), TaggedType::Ref(0)],
                )?;

                Ok(TypedBuiltin::Swap)
            }
            ReducedBuiltin::Rotate3 => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Ref(0), TaggedType::Ref(1), TaggedType::Ref(2)],
                    &[TaggedType::Ref(2), TaggedType::Ref(0), TaggedType::Ref(1)],
                )?;

                Ok(TypedBuiltin::Rotate3)
            }
            ReducedBuiltin::Print => {
                Self::stack_size(stack, 1)?;
                stack.pop();

                Ok(TypedBuiltin::Print)
            }

            ReducedBuiltin::Less => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::Less)
            }
            ReducedBuiltin::Greater => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::Greater)
            }
            ReducedBuiltin::LessEqual => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::LessEqual)
            }
            ReducedBuiltin::GreaterEqual => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::GreaterEqual)
            }
            ReducedBuiltin::Equal => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Ref(0), TaggedType::Ref(0)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::Equal)
            }
            ReducedBuiltin::NotEqual => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Ref(0), TaggedType::Ref(0)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::NotEqual)
            }
            ReducedBuiltin::And => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Type(Type::Bool), TaggedType::Type(Type::Bool)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::And)
            }
            ReducedBuiltin::Or => {
                Self::stack_operation(
                    stack,
                    &[TaggedType::Type(Type::Bool), TaggedType::Type(Type::Bool)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::Or)
            }

            ReducedBuiltin::Assign => {
                Self::stack_size(stack, 2)?;
                let write_value = stack.pop().unwrap();
                let stack_value = stack.pop().unwrap();
                let stack_type = match stack_value {
                    Type::Var(t) => *t,
                    Type::Ptr(p) => p.r#type,
                    _ => {
                        return Err(todo!());
                    }
                };
                if !write_value.can_become(&stack_type) {
                    return Err(todo!());
                }

                Ok(TypedBuiltin::Assign)
            }
            ReducedBuiltin::Read => {
                Self::stack_size(stack, 1);
                let stack_value = stack.pop().unwrap();
                let stack_type = match stack_value {
                    Type::Var(t) => *t,
                    Type::Ptr(p) => p.r#type,
                    _ => todo!(),
                };
                stack.push(stack_type);
                Ok(TypedBuiltin::Read)
            }
            ReducedBuiltin::Input => {
                stack.push(Type::Ptr(Box::new(PtrType {
                    is_const: true,
                    r#type: Type::Char,
                })));
                Ok(TypedBuiltin::Input)
            }
            ReducedBuiltin::Mem => {
                Self::stack_size(stack, 2)?;

                Self::stack_operation(stack, &[TaggedType::Type(Type::I32)], &[]);

                let t = stack.pop().unwrap();
                stack.push(Type::Ptr(Box::new(PtrType {
                    is_const: false,
                    r#type: t,
                })));
                Ok(TypedBuiltin::Mem)
            }

            ReducedBuiltin::Nth(n) => {
                Self::stack_size(stack, 1)?;

                if let Type::Union(v) = stack.pop().unwrap() {
                    if n >= v.types.len() {
                        return Err(todo!());
                    }
                    stack.push(v.types[n].clone());
                }

                Ok(TypedBuiltin::Nth(n))
            }
            ReducedBuiltin::NthWrite(n) => {
                Self::stack_size(stack, 2)?;

                let write = stack.pop().unwrap();

                if let Type::Union(v) = stack.pop().unwrap() {
                    if n >= v.types.len() {
                        return Err(todo!());
                    }

                    if !write.can_become(&v.types[n]) {
                        return Err(todo!());
                    }

                    stack.push(Type::Union(v));
                }

                Ok(TypedBuiltin::NthWrite(n))
            }
            ReducedBuiltin::Union(n) => {
                Self::stack_size(stack, n)?;

                let mut types: Vec<Type> = Vec::new();
                for _ in 0..n {
                    types.push(stack.pop().unwrap());
                }
                types = types.iter().rev().cloned().collect();

                stack.push(Type::Union(Box::new(UnionType { types: types })));
                Ok(TypedBuiltin::Union(n))
            }

            ReducedBuiltin::Record(name) => {
                stack.push(Type::RecordIden(name.clone()));
                Ok(TypedBuiltin::Record(name))
            }
            ReducedBuiltin::RecordType(name) => {
                stack.push(Type::Type(Box::new(Type::RecordIden(name.clone()))));
                Ok(TypedBuiltin::Type(Type::RecordIden(name)))
            }

            ReducedBuiltin::BasicType(t) => {
                stack.push(Type::Type(Box::new(Type::from_basic_type(&t))));
                Ok(TypedBuiltin::Type(Type::from_basic_type(&t)))
            }

            ReducedBuiltin::StringValue(s) => {
                stack.push(Type::Ptr(Box::new(PtrType {
                    is_const: true,
                    r#type: Type::Char,
                })));
                Ok(TypedBuiltin::StringValue(s))
            }
            ReducedBuiltin::I32Value(i) => {
                stack.push(Type::I32);
                Ok(TypedBuiltin::I32Value(i))
            }
            ReducedBuiltin::BoolValue(b) => {
                stack.push(Type::Bool);
                Ok(TypedBuiltin::BoolValue(b))
            }
            ReducedBuiltin::CharValue(c) => {
                stack.push(Type::Char);
                Ok(TypedBuiltin::CharValue(c))
            }

            ReducedBuiltin::DebugPrintStack => Ok(TypedBuiltin::DebugPrintStack),
            ReducedBuiltin::DebugHeapStack => Ok(TypedBuiltin::DebugHeapStack),
        }
    }
}
