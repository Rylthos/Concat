use super::ir::IR;
use crate::{
    ast::typed_node::TypedFuncDeclNode,
    error::ir_error::IRError,
    ir::ir_instructions::{IRInstruction, Label},
};

impl IR {
    pub(crate) fn process_funcs(
        &mut self,
        func_node: &TypedFuncDeclNode,
    ) -> Result<Vec<IRInstruction>, IRError> {
        let mut instructions = Vec::new();

        let mut sections = Vec::new();

        sections.push(vec![IRInstruction::Ret]);
        sections.push(self.process_region(&func_node.region)?);
        sections.push(vec![IRInstruction::Label(Label {
            name: func_node.name.clone(),
        })]);

        for mut section in sections.iter_mut().rev() {
            instructions.append(&mut section);
        }

        Ok(instructions)
    }
}
