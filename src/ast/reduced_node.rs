use super::raw_node::{Literal, RecordDeclNode};
use crate::builtins::basic_types::BasicType;
use crate::builtins::reduced_builtins::ReducedBuiltin;
use crate::lexer::tokens::PositionInfo;

#[derive(Debug, Clone)]
pub enum ReducedAstNode {
    Builtin(PositionInfo, ReducedBuiltin),

    Literal(Literal),
    RecordElementIdentifier(Literal),
    WriteRecordElementIdentifier(Literal),

    Assign(ReducedAssignNode),

    If(ReducedIfNode),
    While(ReducedWhileNode),

    FuncDecl(ReducedFuncDeclNode),
    RecordDecl(RecordDeclNode),
}

#[derive(Debug, Clone)]
pub struct ReducedRegion {
    pub region: Vec<ReducedAstNode>,
}

impl ReducedRegion {
    pub fn new() -> ReducedRegion {
        ReducedRegion { region: Vec::new() }
    }

    pub fn from_vec(value: Vec<ReducedAstNode>) -> ReducedRegion {
        ReducedRegion { region: value }
    }
}

#[derive(Debug, Clone)]
pub struct ReducedIfNode {
    pub position: PositionInfo,
    pub if_region: Vec<(ReducedRegion, ReducedRegion)>,
    pub else_region: ReducedRegion,
}

#[derive(Debug, Clone)]
pub struct ReducedWhileNode {
    pub position: PositionInfo,
    pub condition: ReducedRegion,
    pub region: ReducedRegion,
}

#[derive(Debug, Clone)]
pub struct ReducedAssignNode {
    pub position: PositionInfo,
    pub labels: Vec<String>,
    pub region: ReducedRegion,
}

#[derive(Debug, Clone)]
pub struct ReducedFuncDeclNode {
    pub position: PositionInfo,
    pub name: String,
    pub inputs: Vec<BasicType>,
    pub outputs: Vec<BasicType>,
    pub region: ReducedRegion,
}
