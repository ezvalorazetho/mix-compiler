

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TokenType {
    Eof,

    Identifier,
    StringLiteral,
    Number,
    True,
    False,
    Null,

    Func,
    Let,
    If,
    Else,
    For,
    While,
    Match,
    Case,
    Default,
    Break,
    Async,
    Await,
    Return,
    Struct,
    Public,
    Enum,
    Import,
    Use,
    Alias,
    Is,
    TypeName,
    Continue,
    
    OpenParent,
    CloseParent,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,

    Plus,
    Minus,
    Slash,
    Star,
    Power,
    Percent,
    Equal,
    Not,

    PlusEqual,
    MinusEqual,
    SlashEqual,
    StarEqual,
    PowerEqual,
    PercentEqual,
    DoubleEqual,
    NotEqual,

    And,
    Or,

    SemiColon,
    Colon,
    Arrow,
    DoubleColon,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Dollar,
    At,
    Hash,
    Ampersand,

    Dot,
    Comma,

    Error,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub fn new(kind: TokenType, value: String, file: String, line: usize, column: usize, start: usize, end: usize) -> Self {
        Self {
            kind,
            value,
            file,
            line,
            column,
            start,
            end,
        }
    }
}
