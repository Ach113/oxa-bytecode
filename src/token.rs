#[derive(Clone, Debug)]
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