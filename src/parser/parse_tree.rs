use crate::error::types::{self, ParserError};
use crate::lexer::tokens::{Token, TokenType, Types};

use crate::parser::parser::Parser;

use std::collections::HashMap;

use std::fmt;

#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub token: Token,
    pub name: String,
    pub inputs: Vec<Types>,
    pub outputs: Vec<Types>,
    pub region: Box<ParseTree>,
}

#[derive(Debug, Clone)]
pub enum ParseTree {
    None,
    Element(Token),
    Region(Vec<ParseTree>),
    If(
        Vec<(Token, Box<ParseTree>, Box<ParseTree>)>,
        (Token, Box<ParseTree>),
    ),
    While(Token, Box<ParseTree>, Box<ParseTree>),
    FuncDecl(FuncDecl),
}

impl ParseTree {
    fn fmt_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let padding = |indent| "  ".repeat(indent);

        match self {
            ParseTree::None => write!(f, "{}NONE\n", padding(indent)),
            ParseTree::Element(t) => write!(f, "{}({})", padding(indent), t),
            ParseTree::Region(r) => {
                write!(f, "{}{{\n", padding(indent))?;
                write!(f, "{} {{\n", padding(indent + 1))?;
                for region in r {
                    region.fmt_indent(f, indent + 2)?;
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
            ParseTree::FuncDecl(func) => func.fmt_indent(f, indent),
        }
    }

    pub fn generate_parse_tree<'a>(
        tokens: impl Iterator<Item = &'a Token>,
        functions: &mut HashMap<String, FuncDecl>,
    ) -> Result<ParseTree, ParserError> {
        let mut region: Vec<ParseTree> = Vec::new();

        let mut peekable = tokens.peekable();

        while let Some(&t) = peekable.peek() {
            match t.token_type {
                TokenType::If => {
                    peekable.next();
                    let mut regions = Vec::new();
                    let mut else_region = (t.clone(), Box::new(ParseTree::Region(vec![])));

                    loop {
                        let conditional_tree = Self::generate_parse_tree(
                            Parser::get_condition(&mut peekable)?.iter(),
                            functions,
                        )?;
                        let region_tree = Self::generate_parse_tree(
                            Parser::get_region(&mut peekable)?.iter(),
                            functions,
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
                                            functions,
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
                        functions,
                    )?;
                    let region_tree = Self::generate_parse_tree(
                        Parser::get_region(&mut peekable)?.iter(),
                        functions,
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

                    let input_types = Parser::get_types(&mut peekable)?
                        .iter()
                        .filter(|i| match i {
                            Types::Void => false,
                            _ => true,
                        })
                        .cloned()
                        .collect();

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

                    let output_types = Parser::get_types(&mut peekable)?
                        .iter()
                        .filter(|i| match i {
                            Types::Void => false,
                            _ => true,
                        })
                        .cloned()
                        .collect();

                    let region_tree = Self::generate_parse_tree(
                        Parser::get_region(&mut peekable)?.iter(),
                        functions,
                    )?;

                    functions.insert(
                        function_name.clone(),
                        FuncDecl {
                            token: t.clone(),
                            name: function_name,
                            inputs: input_types,
                            outputs: output_types,
                            region: Box::new(region_tree),
                        },
                    );
                }
                _ => {
                    peekable.next();
                    region.push(ParseTree::Element(t.clone().clone()))
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

        write!(f, "{}{}: {} {{\n", padding(indent), self.name, self.token)?;
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
