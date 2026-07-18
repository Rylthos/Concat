use crate::ast::node::{AstNode, WhileNode};
use crate::error::parser_error::ParserError;
use crate::lexer::tokens::TokenType;

use super::parser::Parser;

impl Parser {
    pub(crate) fn parse_while(&mut self) -> Result<AstNode, ParserError> {
        let position = self.peek().position_info;
        self.consume(TokenType::While)?;

        let condition = self.parse_until(&[TokenType::LeftBrace])?;
        self.consume(TokenType::LeftBrace)?;
        let region = self.parse_until(&[TokenType::RightBrace])?;
        self.consume(TokenType::RightBrace)?;

        Ok(AstNode::While(WhileNode {
            position,
            condition,
            region,
        }))
    }
}
