use super::raw_node::Literal;
use crate::builtins::typed_builtins::TypedBuiltin;
use crate::builtins::types::Type;
use crate::lexer::tokens::PositionInfo;

#[derive(Debug, Clone)]
pub enum TypedAstNode {
    Builtin(PositionInfo, TypedBuiltin),

    Variable(TypedVariableNode),
    FunctionCall(Literal),

    Assign(TypedAssignNode),
    If(TypedIfNode),
    While(TypedWhileNode),
    FuncDecl(TypedFuncDeclNode),
}

#[derive(Debug, Clone)]
pub struct TypedRegion {
    pub region: Vec<TypedAstNode>,
}

impl TypedRegion {
    pub fn new() -> TypedRegion {
        TypedRegion { region: Vec::new() }
    }

    pub fn from_vec(value: Vec<TypedAstNode>) -> TypedRegion {
        TypedRegion { region: value }
    }
}

#[derive(Debug, Clone)]
pub struct TypedIfNode {
    pub position: PositionInfo,
    pub if_region: Vec<(TypedRegion, TypedRegion)>,
    pub else_region: TypedRegion,
}

#[derive(Debug, Clone)]
pub struct TypedWhileNode {
    pub position: PositionInfo,
    pub condition: TypedRegion,
    pub region: TypedRegion,
}

#[derive(Debug, Clone)]
pub struct TypedAssignNode {
    pub position: PositionInfo,
    pub labels: Vec<String>,
    pub region: TypedRegion,
}

#[derive(Debug, Clone)]
pub struct TypedFuncDeclNode {
    pub position: PositionInfo,
    pub name: String,
    pub inputs: Vec<Type>,
    pub outputs: Vec<Type>,
    pub region: TypedRegion,
}

#[derive(Debug, Clone)]
pub struct TypedVariableNode {
    pub position: PositionInfo,
    pub name: String,
    pub depth: usize,
    pub offset: usize,
    pub r#type: Type,
}

#[derive(Debug, Clone)]
pub struct TypedRecordDeclNode {
    pub position: PositionInfo,
    pub name: String,
    pub entries: Vec<(String, Type)>,
}
