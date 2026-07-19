use crate::ast::raw_node::{AstNode, RecordDeclNode, Region};
use crate::error::parser_error::ParserError;
use crate::lexer::tokens::TokenType;

use super::parser::Parser;

impl Parser {
    pub(crate) fn parse_record(&mut self) -> Result<AstNode, ParserError> {
        let pos = self.consume(TokenType::Record)?.position_info;

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

        self.consume(TokenType::LeftBrace)?;
        let mut entries = Vec::new();

        while !self.is_at_end()
            && let t = self.peek()
        {
            match t.token_type {
                TokenType::RightBrace => break,
                _ => {
                    let type_region = self.parse_type()?;
                    let r#type = &self.valid_type_list(Region {
                        region: vec![type_region],
                    })?[0];

                    let identifier_token = self.advance();
                    let identifier = match identifier_token.token_type {
                        TokenType::Identifier(s) => s,
                        _ => {
                            return Err(ParserError::ExpectedIdentifier(
                                identifier_token.position_info,
                            ));
                        }
                    };

                    entries.push((identifier, r#type.clone()));
                }
            }
        }
        self.consume(TokenType::RightBrace)?;

        self.record_names.insert(name.clone());

        Ok(AstNode::RecordDecl(RecordDeclNode {
            position: pos,
            name,
            entries,
        }))
    }
}
