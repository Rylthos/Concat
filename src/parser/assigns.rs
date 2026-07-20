use crate::ast::raw_node::{AssignNode, AstNode};
use crate::error::parser_error::ParserError;
use crate::lexer::tokens::TokenType;

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
                _ => return Err(ParserError::ExpectedIdentifier(pos)),
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
