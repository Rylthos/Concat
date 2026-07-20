use crate::ast::raw_node::{AstNode, IfNode, Region};
use crate::error::parser_error::ParserError;
use crate::lexer::tokens::TokenType;

use super::parser::Parser;

impl Parser {
    pub(crate) fn parse_if(&mut self) -> Result<AstNode, ParserError> {
        let position = self.peek().position_info;
        self.consume(TokenType::If)?;

        let mut if_region = Vec::new();
        let mut else_region = Region::new();

        while !self.is_at_end() {
            let condition = self.parse_until(&[TokenType::LeftBrace])?;
            self.consume(TokenType::LeftBrace)?;
            let region = self.parse_until(&[TokenType::RightBrace])?;
            self.consume(TokenType::RightBrace)?;

            if_region.push((condition, region));

            if self.compare_next(TokenType::Else) {
                self.consume(TokenType::Else)?;

                if self.compare_next(TokenType::If) {
                    self.consume(TokenType::If)?;
                    continue;
                } else {
                    self.consume(TokenType::LeftBrace)?;
                    else_region = self.parse_until(&[TokenType::RightBrace])?;
                    self.consume(TokenType::RightBrace)?;
                    break;
                }
            } else {
                break;
            }
        }

        Ok(AstNode::If(IfNode {
            position,
            if_region,
            else_region,
        }))
    }
}
