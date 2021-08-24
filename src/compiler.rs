use crate::Error;
use crate::scanner::Scanner;
use crate::chunk::{Chunk, OpCode};
use crate::token::{Token, TokenType};
use crate::value::Value;

use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(PartialEq, PartialOrd, Debug)]
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

const BIN: [TokenType; 14] = [TokenType::PLUS, TokenType::MINUS, TokenType::SLASH, TokenType::PERCENT, TokenType::STAR, TokenType::STAR, 
                              TokenType::OR, TokenType::AND, TokenType::EQUAL_EQUAL, TokenType::BANG_EQUAL, TokenType::LESS, TokenType::GREATER,
                              TokenType::GREATER_EQUAL, TokenType::LESS_EQUAL];

#[derive(Default)]
pub struct LocalEnv {
    locals: HashMap<Local, usize>,
    scope_depth: usize // depth 0 => global scope
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Local {
    name: Token,
    depth: usize
}

pub struct Compiler {
    scanner: Scanner,
    env: LocalEnv,
    current: Token,
    previous: Token,
    loop_counter: usize,
    pub chunk: Chunk,
}

impl Compiler {
    pub fn new(source: String) -> Self {
        Compiler {scanner: Scanner::new(source), env: LocalEnv::default(), current: Token::default(), previous: Token::default(), loop_counter: 0, chunk: Chunk::new()}
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
                    println!("{:?}", e);
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
            Err(Error::RUNTIME_ERROR(format!("{} got `{}`", error_message, self.current), self.current.line))
        }
    }

    fn write_byte(&mut self, byte: OpCode) {
        self.chunk.write_chunk(byte, self.previous.line);
    }

    fn write_constant(&mut self, constant: Value) {
        let address = self.chunk.write_value(constant);
        self.write_byte(OpCode::CONSTANT(address));
    }

    pub fn compile(&mut self) -> Result<(), Error> {
        self.advance()?; // consume default
        while !self.check_type(&TokenType::EOF) {
            self.declaration()?;
        }
        self.write_byte(OpCode::RETURN);
        Ok(())
    }

    fn get_precendence(&mut self, token: Token) -> Precendence {
        match token.t {
            TokenType::LEFT_PAREN => Precendence::NONE,
            TokenType::AND => Precendence::AND,
            TokenType::OR => Precendence::OR,
            TokenType::MINUS => Precendence::TERM,
            TokenType::PLUS => Precendence::TERM,
            TokenType::SLASH => Precendence::FACTOR,
            TokenType::STAR => Precendence::FACTOR,
            TokenType::PERCENT => Precendence::FACTOR,
            TokenType::EQUAL_EQUAL => Precendence::EQUALITY,
            TokenType::BANG_EQUAL => Precendence::EQUALITY,
            TokenType::LESS => Precendence::COMPARISON,
            TokenType::GREATER => Precendence::COMPARISON,
            TokenType::LESS_EQUAL => Precendence::COMPARISON,
            TokenType::GREATER_EQUAL => Precendence::COMPARISON,
            TokenType::NUMBER => Precendence::NONE,
            _ => Precendence::NONE
        }
    }

    // Statements
    fn declaration(&mut self) -> Result<(), Error> {
        match self.current.t {
            TokenType::VAR => self.variable_declr(),
            _ => self.statement(),
        }
    }

    fn variable_declr(&mut self) -> Result<(), Error> {
        self.advance()?; // consume `var`
        self.consume(TokenType::IDENTIFIER, "Expect identifier after `var`")?;
        let identifier = self.previous.clone();

        match self.current.t {
            TokenType::EQUAL => {
                self.advance()?;
                self.expression()?;
            },
            _ => self.write_constant(Value::NIL)
        }
        self.consume(TokenType::SEMICOLON, "Expect `;` after statement")?;

        // locals
        if self.env.scope_depth > 0 {
            self.add_local(identifier);
            return Ok(());
        }

        let address = self.chunk.write_value(Value::STRING(identifier.lexeme.clone()));
        self.write_byte(OpCode::DEFINE_GLOBAL(address));
        Ok(())
    }

    fn add_local(&mut self, name: Token) {
        let local = Local {name, depth: self.env.scope_depth};
        let count = self.env.locals.len();
        self.env.locals.insert(local, count);
    }

    fn statement(&mut self) -> Result<(), Error> {
        match self.current.t {
            TokenType::PRINT => self.print_stmt(),
            TokenType::LEFT_BRACE => self.block_stmt(),
            TokenType::IF => self.if_stmt(),
            TokenType::WHILE => self.while_loop(),
            TokenType::BREAK => {
                self.advance()?;
                self.consume(TokenType::SEMICOLON, "Expect `;` after statement")?;
                self.write_byte(OpCode::JMP(0));
                Err(Error::BREAK(self.chunk.code.len() - 1))
            },
            TokenType::CONTINUE => {
                self.advance()?;
                self.consume(TokenType::SEMICOLON, "Expect `;` after statement")?;
                self.write_byte(OpCode::JMP(0));
                Err(Error::CONTINUE(self.chunk.code.len() - 1))
            },
            // expression statements
            _ => self.expression_stmt(),
        }
    }

    fn print_stmt(&mut self) -> Result<(), Error> {
        self.advance()?; // consume `print` token
        self.expression()?; // expression to be printed
        self.write_byte(OpCode::PRINT);
        self.consume(TokenType::SEMICOLON, "Expect `;` after statement")?;
        Ok(())
    }

    fn block_stmt(&mut self) -> Result<(), Error> {
        self.consume(TokenType::LEFT_BRACE, "Expect `{` at the start of block statement")?; // consume `{`
        let mut ret: Result<(), Error> = Ok(());
        self.env.scope_depth += 1;


        while self.current.t != TokenType::RIGHT_BRACE && self.current.t != TokenType::EOF {
            if let Err(e) = self.declaration() {
                match e {
                    Error::BREAK(_) | Error::CONTINUE(_) => {
                        ret = Err(e);
                    },
                    _ => return Err(e)
                }
            }
        }

        self.env.scope_depth -= 1;
        // free stack and local variables
        let locals: Vec<Local> = self.env.locals.keys().cloned().collect();
        let scope_depth = self.env.scope_depth + 1;
        for local in locals.iter().filter(|x| x.depth == scope_depth) {
            self.write_byte(OpCode::POP);
            self.env.locals.remove(&local);
        }
        
        self.consume(TokenType::RIGHT_BRACE, "Expect `}` after block statement")?;
        ret
    }  

    fn if_stmt(&mut self) -> Result<(), Error> {
        self.advance()?; // consume `if`
        let mut ret: Result<(), Error> = Ok(());

        self.expression()?; // conditional statement
        let index = self.chunk.code.len(); // stack address of `if` instruction
        self.write_byte(OpCode::IF(0)); // temp
        self.write_byte(OpCode::POP);
        // body of `if` statement
        if let Err(e) = self.block_stmt() {
            match e {
                Error::BREAK(_) | Error::CONTINUE(_) => ret = Err(e),
                _ => return Err(e)
            }
        }
        let jaddr = self.chunk.code.len(); // jump address

        // `else` statement
        if self.current.t == TokenType::ELSE {
            self.chunk.code[index] = OpCode::IF(jaddr + 1);
            // index for jump instruction after `then` block
            let index = self.chunk.code.len();
            self.write_byte(OpCode::IF(0)); // temp
            self.write_byte(OpCode::POP);

            self.advance()?; // consume `else`
            if let Err(e) = self.block_stmt() {
                match e {
                    Error::BREAK(_) | Error::CONTINUE(_) => ret = Err(e),
                    _ => return Err(e)
                }
            }

            let jaddr = self.chunk.code.len(); // jump address
            self.chunk.code[index] = OpCode::JMP(jaddr);
        } else {
            self.chunk.code[index] = OpCode::IF(jaddr);
        }
        self.write_byte(OpCode::POP);
        ret
    }

    fn while_loop(&mut self) -> Result<(), Error> {
        self.advance()?; // consume `while`
        // handlers for `break` and `continue` statements
        let mut break_: Option<usize> = None;
        let mut continue_: Option<usize> = None;

        let loop_start = self.chunk.code.len();

        self.expression()?; // conditional statement
        let index = self.chunk.code.len(); // index of loop condition
        self.write_byte(OpCode::IF(0)); // temp
        self.write_byte(OpCode::POP);
        // loop body
        if let Err(e) = self.block_stmt() {
            match e {
                Error::BREAK(i) => break_ = Some(i),
                Error::CONTINUE(i) => continue_ = Some(i),
                _ => return Err(e)
            }
        }
        // loop
        self.write_byte(OpCode::JMP(loop_start));

        let jaddr = self.chunk.code.len();
        self.chunk.code[index] = OpCode::IF(jaddr);
        self.write_byte(OpCode::POP);

        if let Some(i) = break_ {
            self.chunk.code[i] = OpCode::JMP(jaddr);
        }
        if let Some(i) = continue_ {
            self.chunk.code[i] = OpCode::JMP(loop_start);
        }
        Ok(())
    }

    fn expression_stmt(&mut self) -> Result<(), Error> {
        self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect `;` after statement")?;
        //self.write_byte(OpCode::POP);
        Ok(())
    }

    // Expressions
    fn expression(&mut self) -> Result<(), Error> {
        self.parse_precendence(Precendence::ASSIGNMENT)
    }

    fn parse_precendence(&mut self, prec: Precendence) -> Result<(), Error> {
        self.advance()?;
        // prefix
        match self.previous.t {
            TokenType::LEFT_PAREN => self.grouping()?,
            TokenType::MINUS | TokenType::BANG => self.unary()?,
            TokenType::NUMBER => self.number()?,
            TokenType::STRING => self.string()?,
            TokenType::IDENTIFIER => {
                self.variable(prec <= Precendence::ASSIGNMENT)?;
            },
            TokenType::TRUE | TokenType::FALSE | TokenType::NIL => self.literal()?,
            _ => {
                return Err(Error::RUNTIME_ERROR(format!("Expected expression got `{}`", self.previous), self.previous.line));
            }
        }
        // infix
        while prec < self.get_precendence(self.current.clone()) {
            self.advance()?;
            if BIN.iter().any(|x| *x == self.previous.t) {
                self.binary()?;
            } else {
                return Err(Error::RUNTIME_ERROR(format!("Invalid infix operator `{}`", self.previous), self.previous.line));
            }
        }
        Ok(())
    }

    fn variable(&mut self, can_assign: bool) -> Result<(), Error> {
        let identifier = self.previous.lexeme.clone();
        let mut is_local = false;

        // resolve local
        let mut address = self.chunk.write_value(Value::STRING(identifier));
        for i in (0..self.env.scope_depth + 1).rev() {
            let local = Local {name: self.previous.clone(), depth: i};
            if let Some(index) = self.env.locals.get(&local) {
                address = *index;
                is_local = true;
                break;
            }
        }

        match self.current.t {
            TokenType::EQUAL => {
                if !can_assign {
                    return Err(Error::COMPILE_ERROR("TypeError: Invalid target for variable assignment".into(), self.previous.line));
                }
                self.advance()?;
                self.expression()?;
                if is_local {
                    self.write_byte(OpCode::SET_LOCAL(address));
                } else {
                    self.write_byte(OpCode::SET_GLOBAL(address));
                }
            },
            _ => {
                if is_local {
                    self.write_byte(OpCode::GET_LOCAL(address));
                } else {
                    self.write_byte(OpCode::GET_GLOBAL(address));
                }
            }
        }
        Ok(())
    }

    fn number(&mut self) -> Result<(), Error> {
        match self.previous.lexeme.parse() {
            Ok(x) => {
                self.write_constant(Value::FLOAT(x));
                Ok(())
            },
            Err(_) => {
                Err(Error::COMPILE_ERROR(format!("TypeError: cannot convert {} to float", self.previous), self.previous.line))
            }
        }
    }

    fn string(&mut self) -> Result<(), Error> {
        self.write_constant(Value::STRING(self.previous.lexeme.clone()));
        Ok(())
    }

    fn literal(&mut self) -> Result<(), Error> {
        match self.previous.t {
            TokenType::TRUE => {
                self.write_constant(Value::BOOL(true));
            },
            TokenType::FALSE => {
                self.write_constant(Value::BOOL(false));
            },
            TokenType::NIL => {
                self.write_constant(Value::NIL);
            },
            _ => {
                return Err(Error::COMPILE_ERROR(format!("Invalid literal of type `{}`", self.previous), self.previous.line));
            }
        }
        Ok(())
    }

    fn grouping(&mut self) -> Result<(), Error> {
        self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect `)` after expression")
    }

    fn unary(&mut self) -> Result<(), Error> {
        let token_type = self.previous.t.clone();

        // compile operand
        self.parse_precendence(Precendence::UNARY)?;

        match token_type {
            TokenType::MINUS => self.write_byte(OpCode::NEGATE),
            TokenType::BANG => self.write_byte(OpCode::BANG),
            _ => {}
        }
        Ok(())
    }

    fn binary(&mut self) -> Result<(), Error> {
        let token_type = self.previous.t.clone();

        let index = self.chunk.code.len(); // for short-circuiting logical OR and AND

        // compile rhs operand
        let prec = self.get_precendence(self.previous.clone());
        self.parse_precendence(prec.next())?;

        match token_type {
            TokenType::PLUS => self.write_byte(OpCode::ADD),
            TokenType::MINUS => self.write_byte(OpCode::SUB),
            TokenType::STAR => self.write_byte(OpCode::MUL),
            TokenType::SLASH => self.write_byte(OpCode::DIV),
            TokenType::PERCENT => self.write_byte(OpCode::REM),
            TokenType::OR => {
                let jaddr = self.chunk.code.len() + 2;
                self.chunk.code.insert(index, OpCode::IFN(jaddr));
                self.write_byte(OpCode::OR);
            },
            TokenType::AND => {
                let jaddr = self.chunk.code.len() + 2;
                self.chunk.code.insert(index, OpCode::IF(jaddr));
                self.write_byte(OpCode::AND);
            },
            TokenType::EQUAL_EQUAL => self.write_byte(OpCode::EQUAL),
            TokenType::LESS => self.write_byte(OpCode::LESS),
            TokenType::GREATER => self.write_byte(OpCode::GREATER),
            TokenType::BANG_EQUAL => {
                self.write_byte(OpCode::EQUAL);
                self.write_byte(OpCode::BANG);
            },
            TokenType::LESS_EQUAL => {
                self.write_byte(OpCode::GREATER);
                self.write_byte(OpCode::BANG);
            },
            TokenType::GREATER_EQUAL => {
                self.write_byte(OpCode::LESS);
                self.write_byte(OpCode::BANG);
            },
            _ => {
                panic!("Unreachable code in binary");
            }
        }
        Ok(())
    }
}