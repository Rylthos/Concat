use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftBrace,
    RightBrace,

    LeftSqBracket,
    RightSqBracket,

    Include,

    Asterisk,
    Exclamation,

    // Arithmetic Operations
    Add,
    Subtract,
    Divide,
    Modulo,

    // Stack Operations
    Drop,
    Duplicate,
    Rotate3,
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

    // Loop
    If,
    Else,
    While,

    // Func
    Func,
    Arrow,

    Record,
    RecordIdentifier(String),

    Union,
    Nth,

    //
    Assignment,
    Assign,
    Read,

    Input,

    //
    Identifier(String),

    Mem,

    Define,

    I32,
    Bool,
    Void,
    Char,
    Const,

    StringValue(String),
    I32Value(i32),
    BoolValue(bool),
    CharValue(char),

    DebugPrintStack,
    DebugHeapStack,
}

#[derive(Debug, Clone)]
pub struct PositionInfo {
    pub line: usize,
    pub column: usize,
    pub file: String,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub string: String,
    pub position_info: PositionInfo,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        line: usize,
        column: usize,
        file: &str,
        string: &str,
    ) -> Token {
        Token {
            token_type,
            string: string.to_string(),
            position_info: PositionInfo {
                line,
                column,
                file: file.to_string(),
            },
        }
    }

    pub fn new_no_file(token_type: TokenType, line: usize, column: usize, string: &str) -> Token {
        Token {
            token_type,
            string: string.to_string(),
            position_info: PositionInfo {
                line,
                column,
                file: "".to_string(),
            },
        }
    }
}

impl PositionInfo {
    pub fn new(line: usize, column: usize, file: &str) -> PositionInfo {
        PositionInfo {
            line,
            column,
            file: file.to_string(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} ({})",
            self.position_info, self.token_type, self.string
        )
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::LeftSqBracket => write!(f, "["),
            TokenType::RightSqBracket => write!(f, "]"),

            TokenType::Include => write!(f, "Include"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Exclamation => write!(f, "!"),
            TokenType::Add => write!(f, "+"),
            TokenType::Subtract => write!(f, "-"),
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

            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::While => write!(f, "while"),

            TokenType::Func => write!(f, "func"),
            TokenType::Arrow => write!(f, "=>"),

            TokenType::Record => write!(f, "record"),
            TokenType::RecordIdentifier(s) => write!(f, "RecordIden({:?})", s),
            TokenType::Union => write!(f, "union"),
            TokenType::Nth => write!(f, "nth"),

            TokenType::Assignment => write!(f, "assign"),
            TokenType::Assign => write!(f, "="),
            TokenType::Read => write!(f, "@"),

            TokenType::Input => write!(f, "input"),

            TokenType::Mem => write!(f, "mem"),

            TokenType::Define => write!(f, "define"),

            TokenType::Identifier(s) => write!(f, "Iden({:?})", s),

            TokenType::I32 => write!(f, "I32"),
            TokenType::Bool => write!(f, "BOOL"),
            TokenType::Void => write!(f, "VOID"),
            TokenType::Char => write!(f, "CHAR"),
            TokenType::Const => write!(f, "CONST"),

            TokenType::StringValue(s) => write!(f, "{:?}", s),
            TokenType::I32Value(i) => write!(f, "{}", i),
            TokenType::BoolValue(b) => write!(f, "{}", b),
            TokenType::CharValue(c) => write!(f, "'{}'", c),

            TokenType::DebugPrintStack => write!(f, "__PRINT_STACK__"),
            TokenType::DebugHeapStack => write!(f, "__PRINT_Heap__"),
        }
    }
}

impl fmt::Display for PositionInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:03}:{:03}", self.file, self.line, self.column)
    }
}
