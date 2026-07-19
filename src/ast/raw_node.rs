use crate::builtins::basic_types::BasicType;
use crate::builtins::builtins::Builtin;
use crate::lexer::tokens::PositionInfo;

#[derive(Debug, Clone)]
pub enum AstNode {
    Builtin(PositionInfo, Builtin),

    Literal(Literal),
    RecordElementIdentifier(Literal),
    WriteRecordElementIdentifier(Literal),

    Assign(AssignNode),

    If(IfNode),
    While(WhileNode),

    FuncDecl(FuncDeclNode),
    RecordDecl(RecordDeclNode),
}

#[derive(Debug, Clone)]
pub struct Region {
    pub region: Vec<AstNode>,
}

impl Region {
    pub fn new() -> Region {
        Region { region: Vec::new() }
    }

    pub fn from_vec(value: Vec<AstNode>) -> Region {
        Region { region: value }
    }
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub position: PositionInfo,
    pub literal: String,
}

#[derive(Debug, Clone)]
pub struct IfNode {
    pub position: PositionInfo,
    pub if_region: Vec<(Region, Region)>,
    pub else_region: Region,
}

#[derive(Debug, Clone)]
pub struct WhileNode {
    pub position: PositionInfo,
    pub condition: Region,
    pub region: Region,
}

#[derive(Debug, Clone)]
pub struct AssignNode {
    pub position: PositionInfo,
    pub labels: Vec<String>,
    pub region: Region,
}

#[derive(Debug, Clone)]
pub struct FuncDeclNode {
    pub position: PositionInfo,
    pub name: String,
    pub inputs: Vec<BasicType>,
    pub outputs: Vec<BasicType>,
    pub region: Region,
}

#[derive(Debug, Clone)]
pub struct RecordDeclNode {
    pub position: PositionInfo,
    pub name: String,
    pub entries: Vec<(String, BasicType)>,
}
