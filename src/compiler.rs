use crate::Error;
use crate::scanner::Scanner;
use crate::chunk::{Chunk, OpCode};
use crate::token::{Token, TokenType};
use crate::value::Value;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd)]
enum Precendence {
    NONE,
    ASSIGNMENT,  // =
    OR,          // or
    AND,         // and
    EQUALITY,    // == !=
    COMPARISON,  // < > <= >=
    TERM,        // + -
    FACTOR,      // * / %
    UNARY,       // ! -
    CALL,        // . ()
    PRIMARY
}

impl Precendence {
    fn next(&self) -> Self {
        match self {
            Precendence::NONE => Precendence::ASSIGNMENT,
            Precendence::ASSIGNMENT => Precendence::OR, 
            Precendence::OR => Precendence::AND,        
            Precendence::AND => Precendence::EQUALITY,       
            Precendence::EQUALITY => Precendence::COMPARISON, 
            Precendence::COMPARISON => Precendence::TERM,  
            Precendence::TERM => Precendence::FACTOR,     
            Precendence::FACTOR => Precendence::UNARY,    
            Precendence::UNARY => Precendence::CALL,    
            Precendence::CALL => Precendence::PRIMARY,      
            Precendence::PRIMARY => Precendence::NONE
        }
    }
}

pub struct Compiler {
    scanner: Scanner,
    current: Token,
    previous: Token,
}

pub fn error(e: &Error) {
    println!("{:?}", e);
}

impl Compiler {
    pub fn new(source: String) -> Self {
        Compiler {scanner: Scanner::new(source), current: Token::default(), previous: Token::default()}
    }

    // helper functions

    fn check_type(&self, tt: &TokenType) -> bool {
        self.current.t == *tt
    }

    fn advance(&mut self) -> Result<(), Error> {
        self.previous = self.current.clone();
        let mut err = false;
        loop {
            match self.scanner.advance() {
                Ok(token) => {
                    self.current = token;
                    break;
                },
                Err(e) => {
                    err = true;
                    error(&e);
                }
            }
        }
        if err {
            return Err(Error::SIGNAL);
        } else {
            return Ok(());
        }
    }

    fn consume(&mut self, tt: TokenType, error_message: &str) -> Result<(), Error> {
        if self.check_type(&tt) {
            self.advance()
        } else {
            Err(Error::RUNTIME_ERROR(format!("{} got {}", error_message, self.current), self.current.line))
        }
    }

    fn write_byte(&self, chunk: &mut Chunk, byte: OpCode) {
        chunk.write_chunk(byte, self.previous.line);
    }

    fn write_constant(&self, chunk: &mut Chunk, constant: Value) {
        let address = chunk.write_value(constant);
        self.write_byte(chunk, OpCode::CONSTANT(address));
    }

    pub fn compile(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
        self.advance()?; // consume default
        self.expression(chunk)?;
        self.consume(TokenType::EOF, "Expect end of expression")?;
        self.write_byte(chunk, OpCode::RETURN);
        Ok(())
    }

    fn get_precendence(&mut self) -> Precendence {
        match self.current.t {
            TokenType::LEFT_PAREN => Precendence::NONE,
            TokenType::AND => Precendence::AND,
            TokenType::OR => Precendence::OR,
            TokenType::MINUS => Precendence::TERM,
            TokenType::PLUS => Precendence::TERM,
            TokenType::SLASH => Precendence::FACTOR,
            TokenType::STAR => Precendence::FACTOR,
            TokenType::PERCENT => Precendence::FACTOR,
            TokenType::NUMBER => Precendence::NONE,
            _ => Precendence::NONE
        }
    }

    fn parse_precendence(&mut self, chunk: &mut Chunk, prec: Precendence) -> Result<(), Error> {
        self.advance()?;
        // prefix
        match self.previous.t {
            TokenType::LEFT_PAREN => {
                self.grouping(chunk)?;
            },
            TokenType::MINUS | TokenType::BANG => {
                self.unary(chunk)?;
            },
            TokenType::NUMBER => {
                self.number(chunk)?;
            },
            TokenType::TRUE | TokenType::FALSE | TokenType::NIL => self.literal(chunk)?,
            _ => {
                return Err(Error::RUNTIME_ERROR(format!("Expected expression got `{}`", self.previous), self.previous.line));
            }
        }
        // infix
        while prec < self.get_precendence() {
            self.advance()?;
            match self.previous.t {
                TokenType::AND | TokenType::OR | TokenType::MINUS | TokenType::PLUS | TokenType::SLASH | TokenType::STAR | TokenType::PERCENT => self.binary(chunk)?,
                _ => return Err(Error::RUNTIME_ERROR(format!("Invalid infix operator `{}`", self.previous), self.previous.line))
            }
        }
        Ok(())
    }

    fn expression(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
        self.parse_precendence(chunk, Precendence::ASSIGNMENT)
    }

    // prefix expression
    fn number(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
        match self.previous.lexeme.parse() {
            Ok(x) => {
                self.write_constant(chunk, Value::FLOAT(x));
                Ok(())
            },
            Err(_) => {
                Err(Error::COMPILE_ERROR(format!("TypeError: cannot convert {} to float", self.previous), self.previous.line))
            }
        }
    }

    fn literal(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
        match self.previous.t {
            TokenType::TRUE => {
                self.write_byte(chunk, OpCode::TRUE);
            },
            TokenType::FALSE => {
                self.write_byte(chunk, OpCode::FALSE);
            },
            TokenType::NIL => {
                self.write_byte(chunk, OpCode::NIL);
            },
            _ => {
                return Err(Error::COMPILE_ERROR(format!("Invalid literal of type `{}`", self.previous), self.previous.line));
            }
        }
        Ok(())
    }

    fn grouping(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
        self.expression(chunk)?;
        self.consume(TokenType::RIGHT_PAREN, "Expect `)` after expression")
    }

    fn unary(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
        let token_type = self.previous.t.clone();

        // compile operand
        self.parse_precendence(chunk, Precendence::UNARY)?;

        match token_type {
            TokenType::MINUS => self.write_byte(chunk, OpCode::NEGATE),
            TokenType::BANG => self.write_byte(chunk, OpCode::BANG),
            _ => {}
        }
        Ok(())
    }

    fn binary(&mut self, chunk: &mut Chunk) -> Result<(), Error> {
        let token_type = self.previous.t.clone();

        // compile rhs operand
        let prec = self.get_precendence();
        self.parse_precendence(chunk, prec.next())?;

        match token_type {
            TokenType::PLUS => {
                self.write_byte(chunk, OpCode::ADD);
            },
            TokenType::MINUS => {
                self.write_byte(chunk, OpCode::SUB);
            },
            TokenType::STAR => {
                self.write_byte(chunk, OpCode::MUL);
            },
            TokenType::SLASH => {
                self.write_byte(chunk, OpCode::DIV);
            },
            TokenType::PERCENT => {
                self.write_byte(chunk, OpCode::REM);
            },
            TokenType::OR => {
                self.write_byte(chunk, OpCode::OR);
            },
            TokenType::AND => {
                self.write_byte(chunk, OpCode::AND);
            },
            _ => {
                // unreachable
            }
        }
        Ok(())
    }
}