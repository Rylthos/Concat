use super::ir::IR;
use crate::{
    ast::typed_node::TypedIfNode, error::ir_error::IRError, ir::ir_instructions::IRInstruction,
};

impl IR {
    pub(crate) fn process_if(
        &mut self,
        if_node: &TypedIfNode,
    ) -> Result<Vec<IRInstruction>, IRError> {
        let mut instructions = Vec::new();

        let end_label = self.new_label("if_end");
        let mut previous_label = self.new_label("if");
        let mut sections = Vec::new();

        sections.push(vec![IRInstruction::Label(end_label.clone())]);

        let else_instructions = self.process_region(&if_node.else_region)?;
        if !else_instructions.is_empty() {
            sections.push(else_instructions);
            sections.push(vec![IRInstruction::Label(previous_label.clone())]);
        } else {
            previous_label = end_label.clone();
        }

        for (c, r) in if_node.if_region.iter().rev() {
            let condition_instructions = self.process_region(&c)?;
            let if_instructions = self.process_region(&r)?;

            sections.push(vec![IRInstruction::Jump(end_label.clone())]);
            sections.push(if_instructions);
            sections.push(vec![IRInstruction::CondFalseJump(previous_label.clone())]);
            sections.push(condition_instructions);
            previous_label = self.new_label("if");
            sections.push(vec![IRInstruction::Label(previous_label.clone())])
        }

        for mut section in sections.iter_mut().rev() {
            instructions.append(&mut section);
        }

        Ok(instructions)
    }
}
