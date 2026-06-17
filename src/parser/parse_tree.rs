use crate::lexer::tokens::{Token, Types};
use std::fmt;

#[derive(Debug, Clone)]
pub enum ParseTree {
    None,
    Element(Token),
    Region(Vec<ParseTree>),
    If(Vec<(Box<ParseTree>, Box<ParseTree>)>, Box<ParseTree>),
    While(Box<ParseTree>, Box<ParseTree>),
    FuncDecl(String, Vec<Types>, Vec<Types>, Box<ParseTree>),
}

impl ParseTree {
    fn fmt_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let padding = |indent| "  ".repeat(indent);

        match self {
            ParseTree::None => write!(f, "{}NONE\n", padding(indent)),
            ParseTree::Element(t) => write!(f, "{}({})", padding(indent), t),
            ParseTree::Region(r) => {
                write!(f, "{}{{\n", padding(indent))?;
                for region in r {
                    region.fmt_indent(f, indent + 1)?;
                    write!(f, "\n")?;
                }
                write!(f, "{}}}", padding(indent))
            }
            ParseTree::If(region_cond, region_else) => {
                write!(f, "{}IF {{\n", padding(indent))?;
                for (c, r) in region_cond {
                    write!(f, "{}{{\n", padding(indent + 1))?;
                    write!(f, "{}COND\n", padding(indent + 2))?;
                    c.fmt_indent(f, indent + 2)?;
                    write!(f, "\n{}REGION\n", padding(indent + 2))?;
                    r.fmt_indent(f, indent + 2)?;
                    write!(f, "\n{}}}\n", padding(indent + 1))?;
                }
                write!(f, "{}ELSE\n", padding(indent + 1))?;
                region_else.fmt_indent(f, indent + 1)?;
                write!(f, "\n{}}}", padding(indent))
            }
            ParseTree::While(region_cond, region) => {
                write!(f, "{}WHILE {{\n", padding(indent))?;
                write!(f, "{}COND\n", padding(indent + 1))?;
                region_cond.fmt_indent(f, indent + 1)?;
                write!(f, "\n{}REGION\n", padding(indent + 1))?;
                region.fmt_indent(f, indent + 1)?;
                write!(f, "\n{}}}", padding(indent))
            }
            ParseTree::FuncDecl(name, input_types, output_types, region) => {
                write!(f, "{}{}: {{\n", padding(indent), name)?;
                write!(f, "{}INPUT: ", padding(indent + 1))?;
                for input in input_types {
                    write!(f, "{} ", input)?;
                }
                write!(f, "\n{}OUTPUT: ", padding(indent + 1))?;
                for output in output_types {
                    write!(f, "{} ", output)?;
                }
                write!(f, "\n{}REGION\n", padding(indent + 1))?;
                region.fmt_indent(f, indent + 1)?;
                write!(f, "\n{}}}", padding(indent))
            }
        }
    }
}

impl fmt::Display for ParseTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_indent(f, 0)
    }
}
