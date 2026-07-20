use crate::{
    ast::{
        reduced_node::ReducedWhileNode,
        typed_node::{TypedAstNode, TypedWhileNode},
    },
    builtins::types::Type,
    error::type_error::TypeError,
    type_checker::type_checker::TaggedType,
};

use super::type_checker::TypeChecker;

use std::collections::HashMap;

impl TypeChecker {
    pub(crate) fn type_check_while(
        &mut self,
        while_node: &ReducedWhileNode,
        stack: &mut Vec<Type>,
        lookup: &HashMap<String, (Type, usize, usize)>,
    ) -> Result<Option<TypedAstNode>, TypeError> {
        let mut stack_copy = stack.clone();
        let condition =
            self.type_check_region(while_node.condition.clone(), &mut stack_copy, lookup)?;
        Self::stack_operation(
            &self.previous_position,
            &mut stack_copy,
            &[TaggedType::Type(Type::Bool)],
            &[],
        )?;

        let region = self.type_check_region(while_node.region.clone(), &mut stack_copy, lookup)?;

        Self::compare_stacks(&self.previous_position, stack, &stack_copy)?;

        Ok(Some(TypedAstNode::While(TypedWhileNode {
            position: while_node.position.clone(),
            condition,
            region,
        })))
    }
}
