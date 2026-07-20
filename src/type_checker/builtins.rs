use crate::{
    builtins::{
        reduced_builtins::ReducedBuiltin,
        typed_builtins::TypedBuiltin,
        types::{PtrType, Type, UnionType},
    },
    error::type_error::TypeError,
    lexer::tokens::PositionInfo,
    type_checker::type_checker::TaggedType,
};

use super::type_checker::TypeChecker;

impl TypeChecker {
    pub(crate) fn type_check_builtin(
        &mut self,
        position: &PositionInfo,
        builtin: &ReducedBuiltin,
        stack: &mut Vec<Type>,
    ) -> Result<TypedBuiltin, TypeError> {
        self.previous_position = position.clone();
        match builtin {
            ReducedBuiltin::Add => {
                Self::stack_size(position, stack, 2)?;
                let v1 = stack.pop().unwrap();
                let v2 = stack.pop().unwrap();

                if let Type::I32 = v1 {
                } else {
                    return Err(TypeError::ExpectedTypeGot(position.clone(), v1, Type::I32));
                }

                if let Type::Ptr(p) = v2 {
                    stack.push(Type::Ptr(p))
                } else if let Type::I32 = v2 {
                    stack.push(Type::I32)
                } else {
                    return Err(TypeError::InvalidAdd(position.clone(), v2));
                }

                Ok(TypedBuiltin::Add)
            }
            ReducedBuiltin::Divide => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::I32)],
                )?;

                Ok(TypedBuiltin::Divide)
            }
            ReducedBuiltin::Modulo => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::I32)],
                )?;

                Ok(TypedBuiltin::Modulo)
            }
            ReducedBuiltin::Multiply => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::I32)],
                )?;

                Ok(TypedBuiltin::Multiply)
            }
            ReducedBuiltin::Subtract => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::I32)],
                )?;

                Ok(TypedBuiltin::Subtract)
            }

            ReducedBuiltin::Drop => {
                Self::stack_operation(position, stack, &[TaggedType::Ref(0)], &[])?;

                Ok(TypedBuiltin::Drop)
            }
            ReducedBuiltin::Duplicate => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Ref(0)],
                    &[TaggedType::Ref(0), TaggedType::Ref(0)],
                )?;

                Ok(TypedBuiltin::Duplicate)
            }
            ReducedBuiltin::Over => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Ref(0), TaggedType::Ref(1)],
                    &[TaggedType::Ref(0), TaggedType::Ref(1), TaggedType::Ref(0)],
                )?;

                Ok(TypedBuiltin::Over)
            }
            ReducedBuiltin::Swap => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Ref(0), TaggedType::Ref(1)],
                    &[TaggedType::Ref(1), TaggedType::Ref(0)],
                )?;

                Ok(TypedBuiltin::Swap)
            }
            ReducedBuiltin::Rotate3 => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Ref(0), TaggedType::Ref(1), TaggedType::Ref(2)],
                    &[TaggedType::Ref(2), TaggedType::Ref(0), TaggedType::Ref(1)],
                )?;

                Ok(TypedBuiltin::Rotate3)
            }
            ReducedBuiltin::Print => {
                Self::stack_size(position, stack, 1)?;
                stack.pop();

                Ok(TypedBuiltin::Print)
            }

            ReducedBuiltin::Less => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::Less)
            }
            ReducedBuiltin::Greater => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::Greater)
            }
            ReducedBuiltin::LessEqual => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::LessEqual)
            }
            ReducedBuiltin::GreaterEqual => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Type(Type::I32), TaggedType::Type(Type::I32)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::GreaterEqual)
            }
            ReducedBuiltin::Equal => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Ref(0), TaggedType::Ref(0)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::Equal)
            }
            ReducedBuiltin::NotEqual => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Ref(0), TaggedType::Ref(0)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::NotEqual)
            }
            ReducedBuiltin::And => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Type(Type::Bool), TaggedType::Type(Type::Bool)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::And)
            }
            ReducedBuiltin::Or => {
                Self::stack_operation(
                    position,
                    stack,
                    &[TaggedType::Type(Type::Bool), TaggedType::Type(Type::Bool)],
                    &[TaggedType::Type(Type::Bool)],
                )?;
                Ok(TypedBuiltin::Or)
            }

            ReducedBuiltin::Assign => {
                Self::stack_size(position, stack, 2)?;
                let write_value = stack.pop().unwrap();
                let stack_value = stack.pop().unwrap();
                let stack_type = match stack_value {
                    Type::Var(t) => *t,
                    Type::Ptr(p) => p.r#type,
                    _ => {
                        return Err(TypeError::InvalidReadWrite(position.clone(), stack_value));
                    }
                };

                if !write_value.can_become(&stack_type) {
                    return Err(TypeError::CannotConvertTypeTo(
                        position.clone(),
                        write_value,
                        stack_type,
                    ));
                }

                Ok(TypedBuiltin::Assign)
            }
            ReducedBuiltin::Read => {
                Self::stack_size(position, stack, 1)?;
                let stack_value = stack.pop().unwrap();
                let stack_type = match stack_value {
                    Type::Var(t) => *t,
                    Type::Ptr(p) => p.r#type,
                    _ => return Err(TypeError::InvalidReadWrite(position.clone(), stack_value)),
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
                Self::stack_size(position, stack, 2)?;

                Self::stack_operation(position, stack, &[TaggedType::Type(Type::I32)], &[])?;

                let value = stack.pop().unwrap();
                let t = match value {
                    Type::Type(a) => a,
                    _ => return Err(TypeError::ExpectedTypeTypeGot(position.clone(), value)),
                };
                stack.push(Type::Ptr(Box::new(PtrType {
                    is_const: false,
                    r#type: *t,
                })));
                Ok(TypedBuiltin::Mem)
            }

            ReducedBuiltin::Nth(n) => {
                Self::stack_size(position, stack, 1)?;

                if let Type::Union(v) = stack.pop().unwrap() {
                    if *n >= v.types.len() {
                        return Err(TypeError::InvalidSize(position.clone(), *n, v.types.len()));
                    }
                    stack.push(v.types[*n].clone());
                }

                Ok(TypedBuiltin::Nth(*n))
            }
            ReducedBuiltin::NthWrite(n) => {
                Self::stack_size(position, stack, 2)?;

                let write = stack.pop().unwrap();

                if let Type::Union(v) = stack.pop().unwrap() {
                    if *n >= v.types.len() {
                        return Err(TypeError::InvalidSize(position.clone(), *n, v.types.len()));
                    }

                    if !write.can_become(&v.types[*n]) {
                        return Err(TypeError::CannotConvertTypeTo(
                            position.clone(),
                            write,
                            v.types[*n].clone(),
                        ));
                    }

                    stack.push(Type::Union(v));
                }

                Ok(TypedBuiltin::NthWrite(*n))
            }
            ReducedBuiltin::Union(n) => {
                Self::stack_size(position, stack, *n)?;

                let mut types: Vec<Type> = Vec::new();
                for _ in 0..*n {
                    types.push(stack.pop().unwrap());
                }
                types = types.iter().rev().cloned().collect();

                stack.push(Type::Union(Box::new(UnionType { types: types })));
                Ok(TypedBuiltin::Union(*n))
            }

            ReducedBuiltin::Record(name) => {
                stack.push(Type::RecordIden(name.clone()));
                Ok(TypedBuiltin::Record(name.to_string()))
            }
            ReducedBuiltin::RecordType(name) => {
                stack.push(Type::Type(Box::new(Type::RecordIden(name.clone()))));
                Ok(TypedBuiltin::Type(Type::RecordIden(name.to_string())))
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
                Ok(TypedBuiltin::StringValue(s.to_string()))
            }
            ReducedBuiltin::I32Value(i) => {
                stack.push(Type::I32);
                Ok(TypedBuiltin::I32Value(*i))
            }
            ReducedBuiltin::BoolValue(b) => {
                stack.push(Type::Bool);
                Ok(TypedBuiltin::BoolValue(*b))
            }
            ReducedBuiltin::CharValue(c) => {
                stack.push(Type::Char);
                Ok(TypedBuiltin::CharValue(*c))
            }

            ReducedBuiltin::DebugPrintStack => Ok(TypedBuiltin::DebugPrintStack),
            ReducedBuiltin::DebugHeapStack => Ok(TypedBuiltin::DebugHeapStack),
        }
    }
}
