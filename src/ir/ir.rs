use crate::{
    ast::typed_node::{TypedAstNode, TypedRegion},
    builtins::{typed_builtins::TypedBuiltin, types::Type},
    config::config::Config,
    error::ir_error::IRError,
    ir::{
        heap_value::HeapValue,
        ir_instructions::{IRInstruction, Label, VariableLookup},
        stack_values::{PointerValue, StackValue},
    },
    type_checker::type_checker::TypedData,
};

use std::collections::HashMap;

pub struct IR {
    config: Config,

    typed_data: TypedData,

    pub(crate) heap: Vec<HeapValue>,

    strings: HashMap<String, usize>,

    label_index: usize,
}

pub struct IRData {
    pub instructions: Vec<IRInstruction>,
    pub initial_heap: Vec<HeapValue>,
}

impl IR {
    pub fn init(config: Config, typed_data: TypedData) -> IR {
        IR {
            config,
            typed_data,
            heap: Vec::new(),
            strings: HashMap::new(),
            label_index: 0,
        }
    }

    pub fn generate_ir_instructions(&mut self) -> Result<IRData, IRError> {
        let region = self.typed_data.main_region.clone();
        let mut instructions = Vec::new();
        instructions.push(IRInstruction::Label(Label {
            name: "main".to_string(),
        }));
        instructions.append(&mut self.process_region(&region)?);
        instructions.push(IRInstruction::Halt);

        let functions = self.typed_data.functions.clone();
        for (_, function) in functions.iter() {
            instructions.append(&mut self.process_funcs(&function)?);
        }

        if self.config.ir_print {
            self.print(&instructions);
        }

        Ok(IRData {
            instructions,
            initial_heap: self.heap.clone(),
        })
    }

    pub(crate) fn new_label(&mut self, label: &str) -> Label {
        let mut name = label.to_string();
        name.push('_');
        name.push_str(&self.label_index.to_string());
        self.label_index += 1;

        Label { name: name }
    }

    fn convert_record(&self, label: &str) -> StackValue {
        if !self.typed_data.records.contains_key(label) {
            unreachable!();
        }

        let record = self.typed_data.records.get(label).unwrap();

        StackValue::Union(Box::new(
            record
                .entries
                .iter()
                .map(|(_, t)| StackValue::from_type(&t))
                .collect(),
        ))
    }

    pub(crate) fn process_region(
        &mut self,
        region: &TypedRegion,
    ) -> Result<Vec<IRInstruction>, IRError> {
        let mut ir = Vec::new();
        for node in region.region.iter() {
            ir.append(&mut self.process_node(&node)?);
        }

        Ok(ir)
    }

    fn process_node(&mut self, node: &TypedAstNode) -> Result<Vec<IRInstruction>, IRError> {
        match node {
            TypedAstNode::Builtin(_, b) => Ok(vec![self.process_builtin(&b)]),
            TypedAstNode::Variable(v) => Ok(vec![IRInstruction::Lookup(VariableLookup {
                depth: v.depth,
                offset: v.offset,
            })]),
            TypedAstNode::FunctionCall(literal) => {
                let function = self.typed_data.functions.get(&literal.literal).unwrap();
                let arguments = function.inputs.len();
                Ok(vec![IRInstruction::Call(
                    arguments,
                    Label {
                        name: literal.literal.clone(),
                    },
                )])
            }
            TypedAstNode::Assign(assign_node) => self.process_assign(assign_node),
            TypedAstNode::If(if_node) => self.process_if(if_node),
            TypedAstNode::While(while_node) => self.process_while(while_node),
            TypedAstNode::FuncDecl(func_decl) => self.process_funcs(func_decl),
        }
    }

    pub(crate) fn process_builtin(&mut self, builtin: &TypedBuiltin) -> IRInstruction {
        match builtin {
            TypedBuiltin::Add => IRInstruction::Add,
            TypedBuiltin::Divide => IRInstruction::Divide,
            TypedBuiltin::Modulo => IRInstruction::Modulo,
            TypedBuiltin::Multiply => IRInstruction::Multiply,
            TypedBuiltin::Subtract => IRInstruction::Subtract,

            TypedBuiltin::Drop => IRInstruction::Drop,
            TypedBuiltin::Duplicate => IRInstruction::Duplicate,
            TypedBuiltin::Over => IRInstruction::Over,
            TypedBuiltin::Swap => IRInstruction::Swap,
            TypedBuiltin::Rotate3 => IRInstruction::Rotate3,
            TypedBuiltin::Print => IRInstruction::Print,

            TypedBuiltin::Less => IRInstruction::Less,
            TypedBuiltin::Greater => IRInstruction::Greater,
            TypedBuiltin::LessEqual => IRInstruction::LessEqual,
            TypedBuiltin::GreaterEqual => IRInstruction::GreaterEqual,
            TypedBuiltin::Equal => IRInstruction::Equal,
            TypedBuiltin::NotEqual => IRInstruction::NotEqual,
            TypedBuiltin::And => IRInstruction::And,
            TypedBuiltin::Or => IRInstruction::Or,

            TypedBuiltin::Assign => IRInstruction::Assign,
            TypedBuiltin::Read => IRInstruction::Read,

            TypedBuiltin::Input => IRInstruction::Input,

            TypedBuiltin::Mem => IRInstruction::Mem,

            TypedBuiltin::Record(name) => IRInstruction::Push(self.convert_record(name)),

            TypedBuiltin::Nth(n) => IRInstruction::Nth(*n),
            TypedBuiltin::NthWrite(n) => IRInstruction::NthWrite(*n),
            TypedBuiltin::Union(n) => IRInstruction::Union(*n),

            TypedBuiltin::Type(t) => IRInstruction::Push(StackValue::Type(t.clone())),

            TypedBuiltin::StringValue(s) => {
                if self.strings.contains_key(s) {
                    IRInstruction::Push(StackValue::Ptr(PointerValue {
                        allocation: *self.strings.get(s).unwrap(),
                        is_constant: true,
                        offset: 0,
                    }))
                } else {
                    let index = self.heap.len();

                    let mut modified_string = s.clone();
                    modified_string.push('\0');

                    self.heap.push(HeapValue {
                        r#type: Type::Char,
                        len: modified_string.len(),
                        data: modified_string.into_boxed_str().into_boxed_bytes(),
                    });

                    IRInstruction::Push(StackValue::Ptr(PointerValue {
                        allocation: index,
                        is_constant: true,
                        offset: 0,
                    }))
                }
            }
            TypedBuiltin::I32Value(i) => IRInstruction::Push(StackValue::I32(*i)),
            TypedBuiltin::BoolValue(b) => IRInstruction::Push(StackValue::Bool(*b)),
            TypedBuiltin::CharValue(c) => IRInstruction::Push(StackValue::Char(*c)),

            TypedBuiltin::DebugPrintStack => IRInstruction::DebugPrintStack,
            TypedBuiltin::DebugHeapStack => IRInstruction::DebugHeapStack,
        }
    }
}
