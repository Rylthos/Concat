use crate::ast::raw_node::{AstNode, Literal, Region};
use crate::builtins::basic_types::{BasicPtrType, BasicType, BasicUnionType};
use crate::builtins::builtins::Builtin;
use crate::config::config::Config;
use crate::error::parser_error::ParserError;
use crate::lexer::tokens::{Token, TokenType};

use std::collections::HashSet;

pub struct Parser {
    config: Config,
    tokens: Vec<Token>,

    index: usize,

    pub(crate) record_names: HashSet<String>,
}

impl Parser {
    pub fn init(config: Config, tokens: Vec<Token>) -> Parser {
        Parser {
            config,
            tokens,
            index: 0,
            record_names: HashSet::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Region, ParserError> {
        let region = self.parse_until(&[])?;

        if self.config.parser_print {
            println!("{:?}", region);
        }

        Ok(region)
    }

    pub(crate) fn peek(&self) -> Token {
        self.tokens[self.index].clone()
    }

    pub(crate) fn next(&mut self) {
        self.index += 1;
    }

    pub(crate) fn advance(&mut self) -> Token {
        let t = self.peek();
        self.next();
        t
    }

    pub(crate) fn consume(&mut self, token: TokenType) -> Result<Token, ParserError> {
        if self.peek().token_type == token {
            Ok(self.advance())
        } else {
            Err(ParserError::ExpectedTokenGot(
                self.peek().position_info.clone(),
                token,
                self.peek().token_type,
            ))
        }
    }

    pub(crate) fn compare_next(&self, token: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token
        }
    }

    pub(crate) fn is_at_end(&self) -> bool {
        return self.index >= self.tokens.len();
    }

    pub(crate) fn parse_until(&mut self, terminators: &[TokenType]) -> Result<Region, ParserError> {
        let mut nodes = Vec::new();
        while !self.is_at_end() {
            if terminators.iter().any(|t| *t == self.peek().token_type) {
                break;
            }

            nodes.push(self.parse_node()?);
        }

        Ok(Region { region: nodes })
    }

    fn parse_node(&mut self) -> Result<AstNode, ParserError> {
        match self.peek().token_type {
            TokenType::If => self.parse_if(),
            TokenType::While => self.parse_while(),
            TokenType::Func => self.parse_func(),
            TokenType::Assignment => self.parse_assign(),
            TokenType::Record => self.parse_record(),

            TokenType::I32
            | TokenType::Bool
            | TokenType::Char
            | TokenType::Void
            | TokenType::LeftSqBracket => Ok(self.parse_type()?),

            _ => self.parse_leaf(),
        }
    }

    pub(crate) fn valid_type_list(&self, region: Region) -> Result<Vec<BasicType>, ParserError> {
        let mut types = Vec::new();

        for (i, r) in (0..).zip(region.region.iter()) {
            match r {
                AstNode::Builtin(p, b) => match b {
                    Builtin::BasicType(t) => match t {
                        BasicType::Void => {
                            if i != region.region.len() - 1 {
                                return Err(ParserError::InvalidTypeListVoid(p.clone()));
                            }
                        }
                        _ => types.push(t.clone()),
                    },
                    Builtin::RecordType(t) => types.push(BasicType::RecordIden(t.clone())),
                    _ => return Err(ParserError::ExpectedTypeGotBuiltin(p.clone(), b.clone())),
                },
                _ => panic!(),
            }
        }
        Ok(types)
    }

    pub(crate) fn parse_type(&mut self) -> Result<AstNode, ParserError> {
        let pos = self.peek().position_info;
        let mut current_type = None;

        while !self.is_at_end() {
            let t = self.peek();
            match &t.token_type {
                TokenType::Identifier(s) => {
                    if let Some(_) = current_type {
                        break;
                    }

                    if self.record_names.contains(&s.clone()) {
                        current_type = Some(BasicType::RecordIden(s.clone()));
                    } else {
                        return Err(ParserError::ExpectedTypeGotToken(
                            t.position_info.clone(),
                            t,
                        ));
                    }
                }
                TokenType::LeftSqBracket => {
                    self.consume(TokenType::LeftSqBracket)?;
                    let types = self.parse_until(&[TokenType::RightSqBracket])?;
                    let types = self.valid_type_list(types)?;

                    current_type = Some(BasicType::Union(Box::new(BasicUnionType { types })));
                }
                TokenType::I32 | TokenType::Bool | TokenType::Char => {
                    if let Some(_) = current_type {
                        break;
                    }

                    current_type = Some(BasicType::from_token(&t));
                }
                TokenType::Void => {
                    current_type = Some(BasicType::Void);
                    self.consume(TokenType::Void)?;
                    break;
                }
                TokenType::Asterisk => {
                    if let Some(value) = current_type {
                        current_type = Some(BasicType::Ptr(Box::new(BasicPtrType {
                            is_const: false,
                            r#type: value,
                        })));
                    } else {
                        return Err(ParserError::ExpectedTypeGotToken(
                            t.position_info.clone(),
                            t,
                        ));
                    }
                }
                TokenType::Const => {
                    if let Some(value) = current_type {
                        match value {
                            BasicType::Ptr(p) => {
                                current_type = Some(BasicType::Ptr(Box::new(BasicPtrType {
                                    is_const: true,
                                    r#type: p.r#type,
                                })))
                            }
                            _ => {
                                return Err(ParserError::ExpectedPointerGotType(
                                    t.position_info.clone(),
                                    value,
                                ));
                            }
                        }
                    } else {
                        return Err(ParserError::ExpectedTypeGotToken(
                            t.position_info.clone(),
                            t,
                        ));
                    }
                }
                _ => {
                    break;
                }
            }
            self.advance();
        }

        if let Some(t) = current_type {
            Ok(AstNode::Builtin(pos, Builtin::BasicType(t)))
        } else {
            Err(ParserError::ExpectedTypeGotToken(pos, self.peek()))
        }
    }

    fn parse_leaf(&mut self) -> Result<AstNode, ParserError> {
        let token = self.advance();

        let pos = token.position_info.clone();

        match token.token_type {
            TokenType::Add => Ok(AstNode::Builtin(pos, Builtin::Add)),
            TokenType::Asterisk => Ok(AstNode::Builtin(pos, Builtin::Multiply)),
            TokenType::Divide => Ok(AstNode::Builtin(pos, Builtin::Divide)),
            TokenType::Modulo => Ok(AstNode::Builtin(pos, Builtin::Modulo)),
            TokenType::Subtract => Ok(AstNode::Builtin(pos, Builtin::Subtract)),

            TokenType::Drop => Ok(AstNode::Builtin(pos, Builtin::Drop)),
            TokenType::Duplicate => Ok(AstNode::Builtin(pos, Builtin::Duplicate)),
            TokenType::Over => Ok(AstNode::Builtin(pos, Builtin::Over)),
            TokenType::Swap => Ok(AstNode::Builtin(pos, Builtin::Swap)),
            TokenType::Rotate3 => Ok(AstNode::Builtin(pos, Builtin::Rotate3)),
            TokenType::Print => Ok(AstNode::Builtin(pos, Builtin::Print)),

            TokenType::Less => Ok(AstNode::Builtin(pos, Builtin::Less)),
            TokenType::Greater => Ok(AstNode::Builtin(pos, Builtin::Greater)),
            TokenType::LessEqual => Ok(AstNode::Builtin(pos, Builtin::LessEqual)),
            TokenType::GreaterEqual => Ok(AstNode::Builtin(pos, Builtin::GreaterEqual)),
            TokenType::Equal => Ok(AstNode::Builtin(pos, Builtin::Equal)),
            TokenType::NotEqual => Ok(AstNode::Builtin(pos, Builtin::NotEqual)),
            TokenType::And => Ok(AstNode::Builtin(pos, Builtin::And)),
            TokenType::Or => Ok(AstNode::Builtin(pos, Builtin::Or)),

            TokenType::Assign => Ok(AstNode::Builtin(pos, Builtin::Assign)),
            TokenType::Read => Ok(AstNode::Builtin(pos, Builtin::Read)),

            TokenType::Input => Ok(AstNode::Builtin(pos, Builtin::Input)),

            TokenType::Mem => Ok(AstNode::Builtin(pos, Builtin::Mem)),

            TokenType::Nth => {
                if self.compare_next(TokenType::Exclamation) {
                    self.consume(TokenType::Exclamation)?;
                    Ok(AstNode::Builtin(pos, Builtin::RawNthWrite))
                } else {
                    Ok(AstNode::Builtin(pos, Builtin::RawNth))
                }
            }
            TokenType::Union => Ok(AstNode::Builtin(pos, Builtin::RawUnion)),

            TokenType::Define => Ok(AstNode::Builtin(pos, Builtin::Define)),

            TokenType::I32Value(i) => Ok(AstNode::Builtin(pos, Builtin::I32Value(i))),
            TokenType::BoolValue(b) => Ok(AstNode::Builtin(pos, Builtin::BoolValue(b))),
            TokenType::CharValue(c) => Ok(AstNode::Builtin(pos, Builtin::CharValue(c))),
            TokenType::StringValue(s) => Ok(AstNode::Builtin(pos, Builtin::StringValue(s))),

            TokenType::RecordIdentifier(s) => {
                if self.compare_next(TokenType::Exclamation) {
                    self.consume(TokenType::Exclamation)?;
                    Ok(AstNode::WriteRecordElementIdentifier(Literal {
                        position: pos,
                        literal: s.clone(),
                    }))
                } else {
                    Ok(AstNode::RecordElementIdentifier(Literal {
                        position: pos,
                        literal: s.clone(),
                    }))
                }
            }
            TokenType::Identifier(s) => {
                if self.record_names.contains(&s) {
                    if self.compare_next(TokenType::Exclamation) {
                        self.consume(TokenType::Exclamation)?;
                        Ok(AstNode::Builtin(pos, Builtin::Record(s.clone())))
                    } else {
                        Ok(AstNode::Builtin(pos, Builtin::RecordType(s.clone())))
                    }
                } else {
                    Ok(AstNode::Literal(Literal {
                        position: pos,
                        literal: s.clone(),
                    }))
                }
            }

            _ => {
                todo!();
                Err(ParserError::UnexpectedToken(token))
            }
        }
    }
}
