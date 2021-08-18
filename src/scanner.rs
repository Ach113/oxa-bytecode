use crate::Error;
use crate::token::*;

pub struct Scanner {
    line: usize,
    start: usize, // start index of token
    current: usize, // current index of token
    source: Vec<char>,
    multi_line_comment: usize, // keeps track of multiline comments
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let chars: Vec<char> = source.chars().collect();
        Scanner {source: chars, line: 1, start: 0, current: 0, multi_line_comment: 0}
    }

    pub fn cell(&self) -> char {
        self.source[self.current]
    }

    // checks if lexer has reached source eof
    pub fn is_eof(&self) -> bool {
        self.current >= self.source.len()
    }

    // checks if passed char is next char (char at index "current + 1")
    pub fn next(&self, next_char: char) -> bool {
        if self.is_eof() {
            false
        } else {
            self.source[self.current + 1] == next_char
        }
    }

    pub fn get_identifier(&mut self) -> Token {
        // identifiers are allowed to contain alphabetic, numeric character and '_' char
        while !self.is_eof() && (self.cell().is_alphabetic() || self.cell().is_digit(10) || self.cell() == '_') {
            self.current += 1;
        }

        self.current -= 1;

        let start = (self.start) as usize;
        let end = (self.current + 1) as usize;
        let identifier: String = self.source[start..end].iter().collect();
        // match the identifier with existing keywords
        match identifier.as_str() {
            "while" => Token::new(identifier.to_string(), TokenType::WHILE, self.line),
            "and" => Token::new(identifier.to_string(), TokenType::AND, self.line),
            "or" => Token::new(identifier.to_string(), TokenType::OR, self.line),
            "class" => Token::new(identifier.to_string(), TokenType::CLASS, self.line),
            "fun" => Token::new(identifier.to_string(), TokenType::FUN, self.line),
            "for" => Token::new(identifier.to_string(), TokenType::FOR, self.line),
            "in" => Token::new(identifier.to_string(), TokenType::IN, self.line),
            "if" => Token::new(identifier.to_string(), TokenType::IF, self.line),
            "else" => Token::new(identifier.to_string(), TokenType::ELSE, self.line),
            "return" => Token::new(identifier.to_string(), TokenType::RETURN, self.line),
            "true" => Token::new(identifier.to_string(), TokenType::TRUE, self.line),
            "false" => Token::new(identifier.to_string(), TokenType::FALSE, self.line),
            "self" => Token::new(identifier.to_string(), TokenType::SELF, self.line),
            "var" => Token::new(identifier.to_string(), TokenType::VAR, self.line),
            "super" => Token::new(identifier.to_string(), TokenType::SUPER, self.line),
            "print" => Token::new(identifier.to_string(), TokenType::PRINT, self.line),
            "nil" => Token::new(identifier.to_string(), TokenType::NIL, self.line),
            "xor" => Token::new(identifier.to_string(), TokenType::XOR, self.line),
            "break" => Token::new(identifier.to_string(), TokenType::BREAK, self.line),
            "continue" => Token::new(identifier.to_string(), TokenType::CONTINUE, self.line),
            "import" => Token::new(identifier.to_string(), TokenType::IMPORT, self.line),
            "as" => Token::new(identifier.to_string(), TokenType::AS, self.line),
            "from" => Token::new(identifier.to_string(), TokenType::FROM, self.line),
            _ => {
                Token::new(identifier.to_string(), TokenType::IDENTIFIER, self.line)
            },
        }
    }

    pub fn get_number(&mut self) -> Result<f64, Error> {
        
        while !self.is_eof() && (self.cell().is_digit(10) || self.cell() == '.') {
            self.current += 1;
        }

        self.current -= 1;

        // index the string
        let start = self.start;
        let end = self.current + 1;
        let s: String = self.source[start..end].iter().collect();
        // check if numeric string can be parsed
        let n = s.trim_end().parse();
        if n.is_err() || s.trim_end().ends_with('.') {
            return Err(Error::COMPILE_ERROR(format!("SyntaxError: Invalid float {}", s), self.line));
        } else {
            Ok(n.unwrap())
        }
    }

    pub fn get_string(&mut self) -> Result<String, Error> {
        // at function call "current" points to first char of string literal
        while !self.next('"') && !self.is_eof() {
            if self.cell() == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }

        // if eof is reached while string has still not been terminated
        if self.is_eof() {
            return Err(Error::COMPILE_ERROR("EOL while scanning string literal".into(), self.line));
        } 

        self.current += 1; // advance further to `consume` end of string literal ('"')

        // index the string
        let start = self.start + 1;
        let end = self.current;
        let string: String = self.source[start..end].iter().collect();
        Ok(string)
    }

    // scans for an individual token at start location (at function call start == current)
    // throws SyntaxError if finds an unexpected character
    pub fn scan_token(&mut self) -> Result<Token, Error> {

        // handles multi-line comments
        while self.multi_line_comment > 0 {
            if self.cell() == '*' && self.next('/') {
                self.multi_line_comment -= 1;
                self.current += 1;
            } else if self.cell() == '/' && self.next('*') {
                self.multi_line_comment += 1;
                self.current += 1;
            } else if self.cell() == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }

        if self.is_eof() { return Ok(Token::new("EOF".to_string(), TokenType::EOF, self.line)); }
        
        let c = self.cell();
        
        match c {
            // single char tokens
            '(' => return Ok(Token::new(c.to_string(), TokenType::LEFT_PAREN, self.line)),
            ')' => return Ok(Token::new(c.to_string(), TokenType::RIGHT_PAREN, self.line)),
            '{' => return Ok(Token::new(c.to_string(), TokenType::LEFT_BRACE, self.line)),
            '}' => return Ok(Token::new(c.to_string(), TokenType::RIGHT_BRACE, self.line)),
            ',' => return Ok(Token::new(c.to_string(), TokenType::COMMA, self.line)),
            '.' => return Ok(Token::new(c.to_string(), TokenType::DOT, self.line)),
            '-' => return Ok(Token::new(c.to_string(), TokenType::MINUS, self.line)),
            '+' => return Ok(Token::new(c.to_string(), TokenType::PLUS, self.line)),
            ';' => return Ok(Token::new(c.to_string(), TokenType::SEMICOLON, self.line)),
            '*' => return Ok(Token::new(c.to_string(), TokenType::STAR, self.line)),
            '%' => return Ok(Token::new(c.to_string(), TokenType::PERCENT, self.line)),
            '[' => return Ok(Token::new(c.to_string(), TokenType::BRA, self.line)),
            ']' => return Ok(Token::new(c.to_string(), TokenType::KET, self.line)),
            // two char tokens
            '!' => return Ok(
                if self.next('=') {
                    self.current += 1;
                    Token::new("!=".to_string(), TokenType::BANG_EQUAL, self.line)
                } else {
                    Token::new(c.to_string(), TokenType::BANG, self.line)
                }
            ),
            '<' => return Ok(
                if self.next('=') {
                    self.current += 1;
                    Token::new("<=".to_string(), TokenType::LESS_EQUAL, self.line)
                } else {
                    Token::new(c.to_string(), TokenType::LESS, self.line)
                }
            ),
            '>' => return Ok(
                if self.next('=') {
                    self.current += 1;
                    Token::new(">=".to_string(), TokenType::GREATER_EQUAL, self.line)
                } else {
                    Token::new(c.to_string(), TokenType::GREATER, self.line)
                }
            ),
            '=' => return Ok(
                if self.next('=') {
                    self.current += 1;
                    Token::new("==".to_string(), TokenType::EQUAL_EQUAL, self.line)
                } else {
                    Token::new(c.to_string(), TokenType::EQUAL, self.line)
                }
            ),
            // special case: '/' stands for division, while // stands for comment
            '/' => {
                if self.next('/') {
                    while !(self.is_eof() || self.cell() == '\n') { 
                        self.current += 1;
                    }
                    self.line += 1;
                    return self.advance();
                } else if self.next('*') {
                    self.multi_line_comment += 1;
                    self.current += 1;
                    return self.advance();
                } else {
                    return Ok(Token::new(c.to_string(), TokenType::SLASH, self.line));
                }
            },
            // strings
            '"' => {
                match self.get_string() {
                    Ok(s) => return Ok(Token::new(s.clone(), TokenType::STRING, self.line)),
                    Err(e) => return Err(e)
                }
            },
            '\n' => {
                self.line += 1;
                self.current += 1;
                return self.advance();
            },
            '\r' | ' ' | '\t' => {
                self.current += 1;
                return self.advance();
            },
            // default 
            _ => {
                if c.is_digit(10) {
                    match self.get_number() {
                        Ok(n) => return Ok(Token::new(n.to_string(), TokenType::NUMBER, self.line)),
                        Err(e) => return Err(e)
                    }
                } else if c.is_alphabetic() {
                    return Ok(self.get_identifier());
                } else {
                    return Err(Error::COMPILE_ERROR(format!("Unexpected character {}", c), self.line));
                }
            },
        }
    }

    pub fn advance(&mut self) -> Result<Token, Error> {
        self.start = self.current;
        let token = self.scan_token();
        self.current += 1;
        return token;
    }
}

