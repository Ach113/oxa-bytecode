use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Default)]
pub struct Token {
    pub lexeme: String,
    pub t: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(lexeme: String, t: TokenType, line: usize) -> Self {
        Token {lexeme, t, line}
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.lexeme == other.lexeme && self.t == other.t
    }
}

impl Eq for Token {}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lexeme.hash(state);
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    // single char tokens
    LEFT_PAREN,
    RIGHT_PAREN, 
    LEFT_BRACE, 
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS, 
    PLUS, 
    SEMICOLON, 
    SLASH,
    PERCENT,
    STAR,
    BRA,
    KET,

    // One or two character tokens.
    BANG, 
    BANG_EQUAL,
    EQUAL, 
    EQUAL_EQUAL,
    GREATER, 
    GREATER_EQUAL,
    LESS, 
    LESS_EQUAL,

    // Literals.
    IDENTIFIER, 
    STRING, 
    NUMBER,

    // Keywords.
    AND, 
    CLASS, 
    ELSE, 
    FALSE, 
    FUN, 
    FOR,
    IN, 
    IF, 
    NIL, 
    OR,
    PRINT, 
    RETURN, 
    SUPER, 
    SELF, 
    TRUE, 
    VAR, 
    WHILE,
    XOR,
    BREAK,
    CONTINUE,
    IMPORT,
    AS,
    FROM,

    EOF
}

impl Default for TokenType {
    fn default() -> Self {TokenType::NIL}
}