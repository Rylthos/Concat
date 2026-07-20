use super::ir::IR;
use crate::{
    ast::typed_node::TypedAssignNode, error::ir_error::IRError, ir::ir_instructions::IRInstruction,
};

impl IR {
    pub(crate) fn process_assign(
        &mut self,
        assign_node: &TypedAssignNode,
    ) -> Result<Vec<IRInstruction>, IRError> {
        let mut instructions = Vec::new();

        let mut sections = Vec::new();

        sections.push(vec![IRInstruction::FrameRemove(assign_node.labels.len())]);
        sections.push(self.process_region(&assign_node.region)?);
        sections.push(vec![IRInstruction::FrameCreate(assign_node.labels.len())]);

        for mut section in sections.iter_mut().rev() {
            instructions.append(&mut section);
        }

        Ok(instructions)
    }
}
