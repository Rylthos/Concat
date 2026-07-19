use crate::ast::raw_node::{AstNode, FuncDeclNode};
use crate::builtins::basic_types::BasicType;
use crate::error::parser_error::ParserError;
use crate::lexer::tokens::TokenType;

use super::parser::Parser;

impl Parser {
    pub(crate) fn parse_func(&mut self) -> Result<AstNode, ParserError> {
        let pos = self.consume(TokenType::Func)?.position_info;

        let name = if !self.is_at_end()
            && let t = self.advance()
        {
            match t.token_type {
                TokenType::Identifier(s) => s.clone(),
                _ => return Err(ParserError::ExpectedIdentifier(t.position_info)),
            }
        } else {
            return Err(ParserError::ExpectedToken(pos));
        };

        let input = self.parse_until(&[TokenType::Arrow])?;
        let input = self.valid_type_list(input)?;
        self.consume(TokenType::Arrow)?;

        let output = self.parse_until(&[TokenType::LeftBrace])?;
        let output = self.valid_type_list(output)?;
        self.consume(TokenType::LeftBrace)?;

        let region = self.parse_until(&[TokenType::RightBrace])?;
        self.consume(TokenType::RightBrace)?;

        let input = input
            .iter()
            .filter(|t| match t {
                BasicType::Void => false,
                _ => true,
            })
            .cloned()
            .collect();

        let output = output
            .iter()
            .filter(|t| match t {
                BasicType::Void => false,
                _ => true,
            })
            .cloned()
            .collect();

        Ok(AstNode::FuncDecl(FuncDeclNode {
            position: pos,
            name,
            inputs: input,
            outputs: output,
            region,
        }))
    }
}
