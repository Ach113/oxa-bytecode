use std::fmt;

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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
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