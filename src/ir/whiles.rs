use super::ir::IR;
use crate::{
    ast::typed_node::TypedWhileNode, error::ir_error::IRError, ir::ir_instructions::IRInstruction,
};

impl IR {
    pub(crate) fn process_while(
        &mut self,
        while_node: &TypedWhileNode,
    ) -> Result<Vec<IRInstruction>, IRError> {
        let mut instructions = Vec::new();

        let end_label = self.new_label("while_end");
        let mut sections = Vec::new();

        sections.push(vec![IRInstruction::Label(end_label.clone())]);

        let condition_label = self.new_label("while");

        sections.push(vec![IRInstruction::Jump(condition_label.clone())]);
        sections.push(self.process_region(&while_node.region)?);
        sections.push(vec![IRInstruction::CondFalseJump(end_label)]);
        sections.push(self.process_region(&while_node.condition)?);
        sections.push(vec![IRInstruction::Label(condition_label.clone())]);

        for mut section in sections.iter_mut().rev() {
            instructions.append(&mut section);
        }

        Ok(instructions)
    }
}
