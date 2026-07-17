use crate::lexer::tokens::PositionInfo;
use crate::parser::parse_tree::ParseTree;
use crate::parser::stack_types::StackType;
use crate::parser::stack_values::StackValue;

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
pub struct DefineDecl {
    pub position_info: PositionInfo,
    pub name: String,
    pub value: StackValue,
}

pub struct ParseInfo {
    pub functions: HashMap<String, FuncDecl>,
    pub records: HashMap<String, RecordDecl>,
    pub constants: HashMap<String, DefineDecl>,
}

impl ParseInfo {
    pub fn new() -> ParseInfo {
        ParseInfo {
            functions: HashMap::new(),
            records: HashMap::new(),
            constants: HashMap::new(),
        }
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

impl DefineDecl {
    pub fn fmt_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        let padding = |indent| "  ".repeat(indent);

        write!(
            f,
            "{}{}: {} | {:?}",
            padding(indent),
            self.name,
            self.position_info,
            self.value
        )
    }
}

impl fmt::Display for DefineDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_indent(f, 0)
    }
}
