use crate::{
    ast::{
        reduced_node::ReducedIfNode,
        typed_node::{TypedAstNode, TypedIfNode},
    },
    builtins::types::Type,
    error::type_error::TypeError,
    type_checker::type_checker::TaggedType,
};

use super::type_checker::TypeChecker;

use std::collections::HashMap;

impl TypeChecker {
    pub(crate) fn type_check_if(
        &mut self,
        if_node: &ReducedIfNode,
        stack: &mut Vec<Type>,
        lookup: &HashMap<String, (Type, usize, usize)>,
    ) -> Result<Option<TypedAstNode>, TypeError> {
        let mut if_region = Vec::new();
        let mut stacks = Vec::new();
        let stack_cond = stack.clone();

        for (c, r) in if_node.if_region.clone() {
            let mut stack_copy = stack_cond.clone();
            let condition = self.type_check_region(c, &mut stack_copy, lookup)?;
            Self::stack_operation(
                &self.previous_position,
                &mut stack_copy,
                &[TaggedType::Type(Type::Bool)],
                &[],
            )?;

            let mut stack_copy2 = stack_copy.clone();

            let region = self.type_check_region(r, &mut stack_copy2, lookup)?;
            stacks.push((self.previous_position.clone(), stack_copy, stack_copy2));
            if_region.push((condition, region));
        }

        if stacks.len() > 1 {
            for i in stacks.windows(2) {
                let (p1, c1, r1) = &i[0];
                let (_, c2, r2) = &i[1];

                Self::compare_stacks(p1, c1, c2)?;
                Self::compare_stacks(p1, r1, r2)?;
            }
        }

        let &mut (ref p, ref mut stack1, ref stack2) = stacks.get_mut(0).unwrap();
        let else_region = self.type_check_region(if_node.else_region.clone(), stack1, lookup)?;
        Self::compare_stacks(&p, &stack1, &stack2)?;

        *stack = stack2.to_vec();

        Ok(Some(TypedAstNode::If(TypedIfNode {
            position: if_node.position.clone(),
            if_region,
            else_region,
        })))
    }
}
