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
        Self::stack_size(&literal.position, &stack, 1)?;

        let stack_value = stack.pop().unwrap();
        let record_name = match stack_value {
            Type::RecordIden(ref s) => s.clone(),
            _ => {
                return Err(TypeError::ExpectedRecordIdenGot(
                    literal.position.clone(),
                    stack_value.clone(),
                ));
            }
        };

        if let Some(record) = self.records.get(&record_name) {
            let entries: Vec<_> = record
                .entries
                .iter()
                .zip(0..)
                .filter(|((name, _), _)| *name == literal.literal)
                .collect();

            if entries.len() == 0 {
                return Err(TypeError::InvalidRecordIdentifier(
                    literal.position.clone(),
                    record_name.to_string(),
                    literal.literal.to_string(),
                ));
            }

            let ((_, stack_type), index) = entries[0];

            stack.push(stack_type.clone());
            Ok(Some(TypedAstNode::Builtin(
                literal.position.clone(),
                TypedBuiltin::Nth(index),
            )))
        } else {
            return Err(TypeError::UnknownRecord(
                literal.position.clone(),
                literal.literal.clone(),
            ));
        }
    }
}
