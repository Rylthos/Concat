#[derive(Debug, Clone)]
pub enum Types {
    String,
    Int,
    Bool,
    Ptr(Box<Types>),
}

#[derive(Debug, Clone)]
pub enum TokenType {
    LeftBrace,
    RightBrace,

    // Arithmetic Operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    // Stack Operations
    Rotate3,
    Duplicate,
    Drop,
    Over,
    Swap,
    Cast,
    Print,

    // Boolean Operations
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,
    Not,

    // Loop
    If,
    Else,
    While,

    Identifier(String),

    Type(Types),

    StringValue(String),
    NumberValue(f64),
    BoolValue(bool),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Token {
        Token { token_type, line }
    }
}
