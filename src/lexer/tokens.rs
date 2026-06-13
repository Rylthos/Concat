#[derive(Debug, Clone)]
pub enum Types {
    String,
    Int,
    Ptr(Box<Types>),
}

#[derive(Debug, Clone)]
pub enum TokenType {
    LeftBrace,
    RightBrace,

    Add,
    Subtract,
    Multiply,
    Divide,

    Cast,
    Print,

    Identifier(String),

    Type(Types),

    True,
    False,
    StringValue(String),
    NumberValue(f64),
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Token {
        Token { token_type, line }
    }
}
