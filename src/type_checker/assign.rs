use crate::{
    ast::{
        reduced_node::ReducedAssignNode,
        typed_node::{TypedAssignNode, TypedAstNode},
    },
    builtins::types::Type,
    error::type_error::TypeError,
};

use super::type_checker::TypeChecker;

use std::collections::HashMap;

impl TypeChecker {
    pub(crate) fn type_check_assign(
        &mut self,
        assign_node: &ReducedAssignNode,
        stack: &mut Vec<Type>,
        lookup: &HashMap<String, (Type, usize, usize)>,
    ) -> Result<Option<TypedAstNode>, TypeError> {
        Self::stack_size(&assign_node.position, stack, assign_node.labels.len())?;
        let stack_copy: Vec<Type> = stack[(stack.len() - assign_node.labels.len())..(stack.len())]
            .iter()
            .cloned()
            .collect();

        let mut variable_lookup = lookup.clone();

        for (_, (_, d, _)) in variable_lookup.iter_mut() {
            *d += 1
        }

        for ((i, v), t) in (0..).zip(assign_node.labels.iter()).zip(stack_copy.iter()) {
            variable_lookup.insert(v.clone(), (t.clone(), 0, i));
        }

        let mut new_stack = Vec::new();
        let region =
            self.type_check_region(assign_node.region.clone(), &mut new_stack, &variable_lookup)?;

        *stack = new_stack;

        Ok(Some(TypedAstNode::Assign(TypedAssignNode {
            position: assign_node.position.clone(),
            labels: assign_node.labels.clone(),
            region,
        })))
    }
}
