use crate::chunk::{Chunk, OpCode};
use crate::Error;
use crate::value::Value;

macro_rules! binary_op {
    ($self:ident, $op:tt) => { $self.stack.pop().unwrap() $op $self.stack.pop().unwrap() }
}

pub struct VM {
    chunk: Chunk,
    ip: usize, // instruction pointer
    stack: Vec<Value>,
}

impl Default for VM {
    fn default() -> Self {
        VM {chunk: Chunk::default(), ip: 0, stack: vec![]}
    }
}

impl VM {

    pub fn set_chunk(&mut self, chunk: Chunk) {
        self.chunk = chunk;
    }

    pub fn interpret(&mut self, debug: bool) -> Result<(), Error> {
        loop {
            let instruction = self.chunk.read_instruction(&mut self.ip);
            if debug {
                instruction.dissassemble_instruction(&self.chunk, self.ip - 1);
            }
            match instruction {
                OpCode::RETURN => {
                    println!("{}", self.stack.pop().unwrap());
                    return Ok(());
                },
                OpCode::CONSTANT(addr) => {
                    let value = self.chunk.read_value(*addr);
                    self.stack.push(value);
                },
                OpCode::NEGATE => {
                    let n = self.stack.len();
                    if n < 1 {
                        return Err(Error::STRING("IndexError: Stack index out of range".into()));
                    }
                    self.stack[n - 1] = (-self.stack[n - 1].clone())?;
                },
                OpCode::ADD => {
                    if self.stack.len() < 2 {
                        return Err(Error::STRING("IndexError: Stack index out of range".into()));
                    }
                    let value = binary_op!(self, +)?;
                    self.stack.push(value);
                },
                OpCode::SUB => {
                    if self.stack.len() < 2 {
                        return Err(Error::STRING("IndexError: Stack index out of range".into()));
                    }
                    let value = binary_op!(self, -)?;
                    self.stack.push(value);
                },
                OpCode::MUL => {
                    if self.stack.len() < 2 {
                        return Err(Error::STRING("IndexError: Stack index out of range".into()));
                    }
                    let value = binary_op!(self, *)?;
                    self.stack.push(value);
                },
                OpCode::DIV => {
                    if self.stack.len() < 2 {
                        return Err(Error::STRING("IndexError: Stack index out of range".into()));
                    }
                    let value = binary_op!(self, /)?;
                    self.stack.push(value);
                },
                OpCode::REM => {
                    if self.stack.len() < 2 {
                        return Err(Error::STRING("IndexError: Stack index out of range".into()));
                    }
                    let value = binary_op!(self, %)?;
                    self.stack.push(value);
                },
                _ => {
                    return Err(Error::COMPILE_ERROR)
                },
            };
        }
    }
}