use crate::error::types::ParserError;
use crate::lexer::tokens::{PositionInfo, Token, TokenType};

use crate::parser::instructions::Instruction;
use crate::parser::intrinsics::Intrinsic;
use crate::parser::parser::Parser;
use crate::parser::stack_types::StackType;
use crate::parser::stack_values::StackValue;

use crate::parser::parse_info::ParseInfo;

use std::collections::HashMap;

use std::fmt;

#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub position_info: PositionInfo,
    pub name: String,
    pub inputs: Vec<StackType>,
    pub outputs: Vec<StackType>,
    pub region: Box<ParseTree>,
}

#[derive(Debug, Clone)]
pub struct RecordDecl {
    pub position_info: PositionInfo,
    pub name: String,
    pub entries: Vec<(String, StackType)>,
}

#[derive(Debug, Clone)]
pub enum ParseTree {
    None,
    Element(PositionInfo, Intrinsic),
    Region(Vec<ParseTree>),
    If(
        Vec<(Token, Box<ParseTree>, Box<ParseTree>)>,
        (Token, Box<ParseTree>),
    ),
    While(Token, Box<ParseTree>, Box<ParseTree>),
    Assign(Token, Vec<String>, Box<ParseTree>),
    FuncDecl(FuncDecl),
    RecordDecl(RecordDecl),
}

impl ParseTree {
    fn fmt_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let padding = |indent| "  ".repeat(indent);

        match self {
            ParseTree::None => write!(f, "{}NONE\n", padding(indent)),
            ParseTree::Element(p, t) => write!(f, "{}({}: {})", padding(indent), p, t),
            ParseTree::Region(r) => {
                write!(f, "{}{{\n", padding(indent))?;
                for region in r {
                    region.fmt_indent(f, indent + 1)?;
                    write!(f, "\n")?;
                }
                write!(f, "{}}}", padding(indent))
            }
            ParseTree::If(region_cond, (t_else, region_else)) => {
                write!(f, "{}IF {{\n", padding(indent))?;
                for (t, c, r) in region_cond {
                    write!(f, "{}{} {{\n", padding(indent + 1), t)?;
                    write!(f, "{}COND\n", padding(indent + 2))?;
                    c.fmt_indent(f, indent + 2)?;
                    write!(f, "\n{}REGION\n", padding(indent + 2))?;
                    r.fmt_indent(f, indent + 2)?;
                    write!(f, "\n{}}}\n", padding(indent + 1))?;
                }
                write!(f, "{}ELSE {}\n", padding(indent + 1), t_else)?;
                region_else.fmt_indent(f, indent + 1)?;
                write!(f, "\n{}}}", padding(indent))
            }
            ParseTree::While(t, region_cond, region) => {
                write!(f, "{}WHILE {} {{\n", padding(indent), t)?;
                write!(f, "{}COND\n", padding(indent + 1))?;
                region_cond.fmt_indent(f, indent + 1)?;
                write!(f, "\n{}REGION\n", padding(indent + 1))?;
                region.fmt_indent(f, indent + 1)?;
                write!(f, "\n{}}}", padding(indent))
            }
            ParseTree::Assign(t, var, region) => {
                write!(f, "{}ASSIGN {} {{\n", padding(indent), t)?;
                write!(f, "{}VAR\n", padding(indent + 1))?;
                for v in var {
                    write!(f, "{}{}\n", padding(indent + 2), v)?;
                }
                write!(f, "{}REGION\n", padding(indent + 1))?;
                region.fmt_indent(f, indent + 2)?;
                write!(f, "\n{}}}", padding(indent))
            }
            ParseTree::FuncDecl(func) => func.fmt_indent(f, indent),
            ParseTree::RecordDecl(record) => record.fmt_indent(f, indent),
        }
    }

    pub fn generate_parse_tree<'a>(
        tokens: impl Iterator<Item = &'a Token>,
        parse_info: &mut ParseInfo,
    ) -> Result<ParseTree, ParserError> {
        let mut region: Vec<ParseTree> = Vec::new();

        let mut peekable = tokens.peekable();

        while let Some(&t) = peekable.peek() {
            match &t.token_type {
                TokenType::If => {
                    peekable.next();
                    let mut regions = Vec::new();
                    let mut else_region = (t.clone(), Box::new(ParseTree::Region(vec![])));

                    loop {
                        let conditional_tree = Self::generate_parse_tree(
                            Parser::get_condition(&mut peekable)?.iter(),
                            parse_info,
                        )?;
                        let region_tree = Self::generate_parse_tree(
                            Parser::get_region(&mut peekable)?.iter(),
                            parse_info,
                        )?;

                        regions.push((
                            t.clone(),
                            Box::new(conditional_tree),
                            Box::new(region_tree),
                        ));

                        if let Some(&t) = peekable.peek() {
                            match t.token_type {
                                TokenType::Else => {
                                    peekable.next();
                                    if let Some(&t2) = peekable.peek() {
                                        match t2.token_type {
                                            TokenType::If => {
                                                peekable.next();
                                                continue;
                                            }
                                            _ => (),
                                        }
                                    }

                                    else_region = (
                                        t.clone(),
                                        Box::new(Self::generate_parse_tree(
                                            Parser::get_region(&mut peekable)?.iter(),
                                            parse_info,
                                        )?),
                                    );

                                    break;
                                }
                                _ => break,
                            }
                        } else {
                            break;
                        }
                    }

                    region.push(ParseTree::If(regions, else_region));
                }
                TokenType::While => {
                    peekable.next();
                    let conditional_tree = Self::generate_parse_tree(
                        Parser::get_condition(&mut peekable)?.iter(),
                        parse_info,
                    )?;
                    let region_tree = Self::generate_parse_tree(
                        Parser::get_region(&mut peekable)?.iter(),
                        parse_info,
                    )?;

                    region.push(ParseTree::While(
                        t.clone(),
                        Box::new(conditional_tree),
                        Box::new(region_tree),
                    ))
                }
                TokenType::Func => {
                    peekable.next();
                    let function_name = if let Some(t) = peekable.next() {
                        match &t.token_type {
                            TokenType::Identifier(s) => s.clone(),
                            _ => {
                                return Err(ParserError::InvalidFunctionDef(
                                    t.position_info.clone(),
                                    t.token_type.clone(),
                                ));
                            }
                        }
                    } else {
                        return Err(ParserError::ExpectedToken(
                            t.position_info.clone(),
                            TokenType::Identifier("".to_string()),
                        ));
                    };

                    let input_types = Parser::get_types(&mut peekable, &mut parse_info.records)?;

                    if let Some(t) = peekable.next() {
                        match &t.token_type {
                            TokenType::Arrow => (),
                            _ => {
                                return Err(ParserError::ExpectedToken(
                                    t.position_info.clone(),
                                    TokenType::Arrow,
                                ));
                            }
                        }
                    } else {
                        return Err(ParserError::ExpectedToken(
                            t.position_info.clone(),
                            TokenType::Arrow,
                        ));
                    };

                    let output_types = Parser::get_types(&mut peekable, &parse_info.records)?;
                    let region_tree = Self::generate_parse_tree(
                        Parser::get_region(&mut peekable)?.iter(),
                        parse_info,
                    )?;

                    parse_info.functions.insert(
                        function_name.clone(),
                        FuncDecl {
                            position_info: t.position_info.clone(),
                            name: function_name,
                            inputs: input_types,
                            outputs: output_types,
                            region: Box::new(region_tree),
                        },
                    );
                }
                TokenType::Record => {
                    peekable.next();
                    let record_name = if let Some(t) = peekable.next() {
                        match &t.token_type {
                            TokenType::Identifier(s) => s.clone(),
                            _ => {
                                return Err(ParserError::InvalidFunctionDef(
                                    t.position_info.clone(),
                                    t.token_type.clone(),
                                ));
                            }
                        }
                    } else {
                        return Err(ParserError::ExpectedToken(
                            t.position_info.clone(),
                            TokenType::Identifier("".to_string()),
                        ));
                    };

                    if let Some(t) = peekable.next() {
                        match &t.token_type {
                            TokenType::LeftBrace => {}
                            _ => {
                                return Err(ParserError::ExpectedTokenGot(
                                    t.position_info.clone(),
                                    TokenType::LeftBrace,
                                    t.token_type.clone(),
                                ));
                            }
                        }
                    } else {
                        return Err(ParserError::ExpectedToken(
                            t.position_info.clone(),
                            TokenType::LeftBrace,
                        ));
                    };

                    let mut elements: Vec<(String, StackType)> = Vec::new();

                    while let Some(&t) = peekable.peek() {
                        match &t.token_type {
                            TokenType::RightBrace => break,
                            _ => {
                                let next_type =
                                    Parser::get_type(&mut peekable, &parse_info.records)?;

                                if let Some(t) = peekable.next() {
                                    match &t.token_type {
                                        TokenType::Identifier(s) => {
                                            elements.push((s.clone(), next_type))
                                        }
                                        _ => {
                                            return Err(ParserError::ExpectedTokenGot(
                                                t.position_info.clone(),
                                                TokenType::Identifier("".to_string()),
                                                t.token_type.clone(),
                                            ));
                                        }
                                    }
                                } else {
                                    return Err(ParserError::ExpectedToken(
                                        t.position_info.clone(),
                                        t.token_type.clone(),
                                    ));
                                }
                            }
                        }
                    }

                    if let Some(t) = peekable.next() {
                        match &t.token_type {
                            TokenType::RightBrace => {}
                            _ => {
                                return Err(ParserError::ExpectedTokenGot(
                                    t.position_info.clone(),
                                    TokenType::RightBrace,
                                    t.token_type.clone(),
                                ));
                            }
                        }
                    } else {
                        return Err(ParserError::ExpectedToken(
                            t.position_info.clone(),
                            TokenType::RightBrace,
                        ));
                    };

                    parse_info.records.insert(
                        record_name.clone(),
                        RecordDecl {
                            position_info: t.position_info.clone(),
                            name: record_name,
                            entries: elements,
                        },
                    );
                }
                TokenType::Assignment => {
                    peekable.next();
                    let variable_list = Parser::get_identifier_list(&mut peekable)?
                        .iter()
                        .map(|t| match &t.token_type {
                            TokenType::Identifier(s) => s.clone(),
                            _ => unreachable!(),
                        })
                        .collect();

                    let region_tree = Self::generate_parse_tree(
                        Parser::get_region(&mut peekable)?.iter(),
                        parse_info,
                    )?;

                    region.push(ParseTree::Assign(
                        t.clone(),
                        variable_list,
                        Box::new(region_tree),
                    ));
                }
                TokenType::RecordIdentifier(s) => {
                    peekable.next();
                    if let Some(t) = peekable.peek() {
                        match t.token_type {
                            TokenType::Exclamation => {
                                region.push(ParseTree::Element(
                                    t.position_info.clone(),
                                    Intrinsic::WriteRecordIdentifier(s.clone()),
                                ));

                                peekable.next();

                                continue;
                            }
                            _ => {}
                        }
                    }

                    region.push(ParseTree::Element(
                        t.position_info.clone(),
                        Intrinsic::RecordIdentifier(s.clone()),
                    ))
                }
                TokenType::Identifier(s) => {
                    peekable.next();
                    if parse_info.records.contains_key(&s.clone()) {
                        if let Some(t) = peekable.peek() {
                            match t.token_type {
                                TokenType::Exclamation => {
                                    region.push(ParseTree::Element(
                                        t.position_info.clone(),
                                        Intrinsic::Record(s.clone()),
                                    ));

                                    peekable.next();

                                    continue;
                                }
                                _ => {}
                            }
                        }
                        region.push(ParseTree::Element(
                            t.position_info.clone(),
                            Intrinsic::StackType(Parser::create_union_type(
                                &parse_info.records.get(s).unwrap(),
                            )),
                        ));
                    } else if parse_info.functions.contains_key(&s.clone()) {
                        region.push(ParseTree::Element(
                            t.position_info.clone(),
                            Intrinsic::FuncIdentifier(s.clone()),
                        ));
                    } else {
                        region.push(ParseTree::Element(
                            t.position_info.clone(),
                            Intrinsic::VariableIdentifier(s.clone()),
                        ));
                    }
                }
                TokenType::Nth => {
                    peekable.next();

                    let previous: usize = if let Some(ParseTree::Element(_, i)) = region.pop() {
                        if let Intrinsic::I32Value(v) = i {
                            v as usize
                        } else {
                            todo!("Expected Int: {}", i)
                        }
                    } else {
                        todo!("Expected Constant Int")
                    };

                    let intrinsic = if let Some(t) = peekable.peek() {
                        match t.token_type {
                            TokenType::Exclamation => {
                                peekable.next();
                                Intrinsic::NthWrite(previous)
                            }
                            _ => Intrinsic::Nth(previous),
                        }
                    } else {
                        Intrinsic::Nth(previous)
                    };

                    region.push(ParseTree::Element(t.position_info.clone(), intrinsic));
                }
                TokenType::LeftSqBracket => {
                    let parsed_type = Parser::get_type(&mut peekable, &parse_info.records)?;

                    region.push(ParseTree::Element(
                        t.position_info.clone(),
                        Intrinsic::StackType(parsed_type),
                    ));
                }
                TokenType::Union => {
                    peekable.next();

                    let previous: i32 = if let Some(ParseTree::Element(_, i)) = region.pop() {
                        if let Intrinsic::I32Value(v) = i {
                            v
                        } else {
                            todo!("Expected Int: {}", i)
                        }
                    } else {
                        todo!("Expected Constant Int")
                    };

                    region.push(ParseTree::Element(
                        t.position_info.clone(),
                        Intrinsic::Union(previous as usize),
                    ));
                }
                TokenType::I32 | TokenType::Bool | TokenType::Char => {
                    let parsed_type = Parser::get_type(&mut peekable, &parse_info.records)?;

                    region.push(ParseTree::Element(
                        t.position_info.clone(),
                        Intrinsic::StackType(parsed_type),
                    ));
                }
                TokenType::Define => {
                    let raw_name = region.pop();
                    let raw_value = region.pop();

                    let name =
                        if let Some(ParseTree::Element(_, Intrinsic::VariableIdentifier(s))) =
                            raw_name
                        {
                            s
                        } else {
                            return Err(ParserError::InvalidDefine(t.position_info.clone()));
                        };

                    let value = if let Some(ParseTree::Element(_, i)) = raw_value {
                        i
                    } else {
                        return Err(ParserError::InvalidDefine(t.position_info.clone()));
                    };

                    if let Instruction::Push(v) = Parser::convert_intrinsic(
                        &value,
                        &HashMap::new(),
                        &HashMap::new(),
                        &HashMap::new(),
                    ) {
                        parse_info.constants.insert(name, v);
                    } else {
                        return Err(ParserError::InvalidDefine(t.position_info.clone()));
                    }

                    peekable.next();
                }
                _ => {
                    peekable.next();
                    region.push(ParseTree::Element(
                        t.position_info.clone(),
                        Parser::convert_token(&t),
                    ))
                }
            }
        }

        return Ok(ParseTree::Region(region));
    }
}

impl fmt::Display for ParseTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_indent(f, 0)
    }
}

impl FuncDecl {
    pub fn fmt_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let padding = |indent| "  ".repeat(indent);

        write!(
            f,
            "{}{}: {} {{\n",
            padding(indent),
            self.name,
            self.position_info
        )?;
        write!(f, "{}INPUT: ", padding(indent + 1))?;
        for input in self.inputs.iter() {
            write!(f, "{} ", input)?;
        }
        write!(f, "\n{}OUTPUT: ", padding(indent + 1))?;
        for output in self.outputs.iter() {
            write!(f, "{} ", output)?;
        }
        write!(f, "\n{}REGION\n", padding(indent + 1))?;
        self.region.fmt_indent(f, indent + 1)?;
        write!(f, "\n{}}}", padding(indent))
    }
}

impl fmt::Display for FuncDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_indent(f, 0)
    }
}

impl RecordDecl {
    pub fn fmt_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let padding = |indent| "  ".repeat(indent);

        write!(
            f,
            "{}{}: {} {{\n",
            padding(indent),
            self.name,
            self.position_info
        )?;
        for (name, stack_type) in self.entries.iter() {
            write!(f, "{}{}: {}\n", padding(indent + 1), name, stack_type)?;
        }
        write!(f, "{}}}", padding(indent))
    }
}

impl fmt::Display for RecordDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_indent(f, 0)
    }
}
