use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Types {
    String,
    I32,
    Bool,
    Void,
    Type,
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

    // Func
    Func,
    Arrow,

    //
    Assignment,
    Assign,
    Read,

    //
    Identifier(String),

    Type(Types),

    StringValue(String),
    I32(i32),
    BoolValue(bool),
}

#[derive(Debug, Clone)]
pub struct PositionInfo {
    pub line: usize,
    pub column: usize,
    pub string: String,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub position_info: PositionInfo,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize, string: &str) -> Token {
        Token {
            token_type,
            position_info: PositionInfo {
                line,
                column,
                string: string.to_string(),
            },
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:03}:{:03}: {}",
            self.position_info.line, self.position_info.column, self.token_type
        )
    }
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Types::String => write!(f, "STRING"),
            Types::I32 => write!(f, "I32"),
            Types::Bool => write!(f, "BOOL"),
            Types::Void => write!(f, "VOID"),
            Types::Type => write!(f, "TYPE"),
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::Add => write!(f, "+"),
            TokenType::Subtract => write!(f, "-"),
            TokenType::Multiply => write!(f, "*"),
            TokenType::Divide => write!(f, "/"),
            TokenType::Modulo => write!(f, "%"),
            TokenType::Rotate3 => write!(f, "rot3"),
            TokenType::Duplicate => write!(f, "dup"),
            TokenType::Drop => write!(f, "drop"),
            TokenType::Over => write!(f, "over"),
            TokenType::Swap => write!(f, "swap"),
            TokenType::Print => write!(f, "print"),
            TokenType::Less => write!(f, "<"),
            TokenType::Greater => write!(f, ">"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Equal => write!(f, "=="),
            TokenType::NotEqual => write!(f, "!="),
            TokenType::And => write!(f, "&&"),
            TokenType::Or => write!(f, "||"),
            TokenType::Not => write!(f, "!"),

            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::While => write!(f, "while"),

            TokenType::Func => write!(f, "func"),
            TokenType::Arrow => write!(f, "=>"),

            TokenType::Assignment => write!(f, "assign"),
            TokenType::Assign => write!(f, "="),
            TokenType::Read => write!(f, "@"),

            TokenType::Identifier(s) => write!(f, "Iden({:?})", s),

            TokenType::Type(t) => write!(f, "{}", t),
            TokenType::StringValue(s) => write!(f, "{:?}", s),
            TokenType::I32(i) => write!(f, "{}", i),
            TokenType::BoolValue(b) => write!(f, "{}", b),
        }
    }
}
