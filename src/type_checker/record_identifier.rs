use crate::{
    ast::{raw_node::Literal, typed_node::TypedAstNode},
    builtins::{typed_builtins::TypedBuiltin, types::Type},
    error::type_error::TypeError,
};

use super::type_checker::TypeChecker;

impl TypeChecker {
    pub(crate) fn type_check_record_identifier(
        &mut self,
        literal: &Literal,
        stack: &mut Vec<Type>,
    ) -> Result<Option<TypedAstNode>, TypeError> {
        Self::stack_size(&stack, 1)?;

        let stack_value = stack.pop().unwrap();
        let record_name = match stack_value {
            Type::RecordIden(ref s) => s.clone(),
            _ => todo!("Expected Record Iden got {stack_value}"),
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

            stack.push(stack_type.clone());
            Ok(Some(TypedAstNode::Builtin(
                literal.position.clone(),
                TypedBuiltin::Nth(index),
            )))
        } else {
            todo!()
        }
    }
}
