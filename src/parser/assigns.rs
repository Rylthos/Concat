use crate::ast::node::{AssignNode, AstNode, FuncDeclNode, IfNode, RecordDeclNode, Region};
use crate::builtins::basic_types::BasicType;
use crate::builtins::builtins::Builtin;
use crate::error::parser_error::ParserError;
use crate::lexer::tokens::{Token, TokenType};

use super::parser::Parser;

impl Parser {
    pub(crate) fn parse_assign(&mut self) -> Result<AstNode, ParserError> {
        let pos = self.consume(TokenType::Assignment)?.position_info;

        let identifiers = self.parse_until(&[TokenType::LeftBrace])?;
        self.consume(TokenType::LeftBrace)?;

        let mut labels = Vec::new();
        for i in identifiers.region {
            match i {
                AstNode::Literal(l) => labels.push(l.literal),
                _ => todo!(),
            }
        }

        let region = self.parse_until(&[TokenType::RightBrace])?;
        self.consume(TokenType::RightBrace)?;

        Ok(AstNode::Assign(AssignNode {
            position: pos,
            labels,
            region,
        }))
    }
}
