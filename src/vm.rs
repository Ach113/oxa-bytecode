use crate::chunk::{Chunk, OpCode};
use crate::Error;
use crate::value::Value;

use std::collections::HashMap;

macro_rules! binary_op {
    ($self:ident, $op:tt) => {{ 
        let b = $self.stack.pop().unwrap();
        let a = $self.stack.pop().unwrap();
        a $op b
    }}
}

pub struct VM {
    chunk: Chunk,
    ip: usize, // instruction pointer
    stack: Vec<Value>,
    symbol_table: HashMap<String, Value>
}

impl Default for VM {
    fn default() -> Self {
        VM {chunk: Chunk::default(), ip: 0, stack: vec![], symbol_table: HashMap::new()}
    }
}

impl VM {

    pub fn set_chunk(&mut self, chunk: Chunk) {
        self.chunk = chunk;
    }

    pub fn execute(&mut self, debug: bool) -> Result<(), Error> {
        if debug {
            println!("------------------------------");
            self.chunk.dissassemble_chunk();
            println!("------------------------------");
        }
        if self.chunk.code.len() == 0 {
            return Ok(());
        }
        loop {
            let instruction = self.chunk.read_instruction(&mut self.ip);
            match instruction {
                OpCode::RETURN => {
                    assert_eq!(0, self.stack.len());
                    return Ok(());
                },
                OpCode::POP => { self.stack.pop(); },
                OpCode::PRINT => println!("{}", self.stack.pop().unwrap()),
                OpCode::CONSTANT(addr) => {
                    let value = self.chunk.read_value(*addr);
                    self.stack.push(value);
                },
                OpCode::DEFINE_GLOBAL(addr) => {
                    if let Value::STRING(s) = self.chunk.read_value(*addr) {
                        self.symbol_table.insert(s, self.stack.pop().unwrap());
                    } else {
                        return Err(Error::RUNTIME_ERROR("NameError: Invalid identifier".into(), self.chunk.get_line(self.ip)));
                    }
                },
                OpCode::GET_GLOBAL(addr) => {
                    if let Value::STRING(s) = self.chunk.read_value(*addr) {
                        if !self.symbol_table.contains_key(&s) {
                            return Err(Error::RUNTIME_ERROR(format!("NameError: undefined variable `{}`", s), self.chunk.get_line(self.ip)));
                        }
                        self.stack.push(self.symbol_table.get(&s).unwrap().clone());
                    }
                },
                OpCode::SET_GLOBAL(addr) => {
                    if let Value::STRING(s) = self.chunk.read_value(*addr) {
                        if !self.symbol_table.contains_key(&s) {
                            return Err(Error::RUNTIME_ERROR(format!("NameError: undefined variable `{}`", s), self.chunk.get_line(self.ip)));
                        }
                        self.symbol_table.insert(s, self.stack.pop().unwrap());
                    }
                },
                OpCode::GET_LOCAL(addr) => {
                    let val = self.stack[*addr].clone();
                    self.stack.push(val);
                },
                OpCode::SET_LOCAL(addr) => {
                    let val = self.stack.last().unwrap().clone();
                    self.stack[*addr] = val;
                },
                OpCode::NEGATE => {
                    let n = self.stack.len();
                    if n < 1 {
                        return Err(Error::RUNTIME_ERROR("IndexError: Stack index out of range".into(), self.chunk.get_line(self.ip)));
                    }
                    let value = -self.stack[n - 1].clone();
                    if let Ok(x) = value {
                        self.stack[n - 1] = x.clone();
                    } else {
                        return Err(Error::RUNTIME_ERROR("TypeError: Unsupported operand types for `-`".into(), self.chunk.get_line(self.ip)));
                    }   
                },
                OpCode::BANG => {
                    let n = self.stack.len();
                    if n < 1 {
                        return Err(Error::RUNTIME_ERROR("IndexError: Stack index out of range".into(), self.chunk.get_line(self.ip)));
                    }
                    let value = self.stack[n - 1].clone();
                    match value {
                        Value::BOOL(x) => self.stack[n - 1] = Value::BOOL(!x),
                        _ => return Err(Error::RUNTIME_ERROR("TypeError: Unsupported operand types for `!`".into(), self.chunk.get_line(self.ip)))
                    }  
                },
                OpCode::ADD => {
                    if self.stack.len() < 2 {
                        return Err(Error::RUNTIME_ERROR("IndexError: Stack index out of range".into(), self.chunk.get_line(self.ip)));
                    }
                    let value = binary_op!(self, +);
                    if let Ok(x) = value {
                        self.stack.push(x.clone());
                    } else {
                        return Err(Error::RUNTIME_ERROR("TypeError: Unsupported operand types for `+`".into(), self.chunk.get_line(self.ip)));
                    }
                },
                OpCode::OR => {
                    if self.stack.len() < 2 {
                        return Err(Error::RUNTIME_ERROR("IndexError: Stack index out of range".into(), self.chunk.get_line(self.ip)));
                    }
                    let value = binary_op!(self, |);
                    if let Ok(x) = value {
                        self.stack.push(x.clone());
                    } else {
                        return Err(Error::RUNTIME_ERROR("TypeError: Unsupported operand types for `or`".into(), self.chunk.get_line(self.ip)));
                    }
                },
                OpCode::AND => {
                    if self.stack.len() < 2 {
                        return Err(Error::RUNTIME_ERROR("IndexError: Stack index out of range".into(), self.chunk.get_line(self.ip)));
                    }
                    let value = binary_op!(self, &);
                    if let Ok(x) = value {
                        self.stack.push(x.clone());
                    } else {
                        return Err(Error::RUNTIME_ERROR("TypeError: Unsupported operand types for `and`".into(), self.chunk.get_line(self.ip)));
                    }
                },
                OpCode::SUB => {
                    if self.stack.len() < 2 {
                        return Err(Error::RUNTIME_ERROR("IndexError: Stack index out of range".into(), self.chunk.get_line(self.ip)));
                    }
                    let value = binary_op!(self, -);
                    if let Ok(x) = value {
                        self.stack.push(x.clone());
                    } else {
                        return Err(Error::RUNTIME_ERROR("TypeError: Unsupported operand types for `-`".into(), self.chunk.get_line(self.ip)));
                    }
                },
                OpCode::MUL => {
                    if self.stack.len() < 2 {
                        return Err(Error::RUNTIME_ERROR("IndexError: Stack index out of range".into(), self.chunk.get_line(self.ip)));
                    }
                    let value = binary_op!(self, *);
                    if let Ok(x) = value {
                        self.stack.push(x.clone());
                    } else {
                        return Err(Error::RUNTIME_ERROR("TypeError: Unsupported operand types for `*`".into(), self.chunk.get_line(self.ip)));
                    }
                },
                OpCode::DIV => {
                    if self.stack.len() < 2 {
                        return Err(Error::RUNTIME_ERROR("IndexError: Stack index out of range".into(), self.chunk.get_line(self.ip)));
                    }
                    let value = binary_op!(self, /);
                    match value {
                        Ok(x) => self.stack.push(x.clone()),
                        Err(e) => {
                            match e {
                                Error::DIVIDE_BY_ZERO => return Err(Error::RUNTIME_ERROR("DivideByZero Error".into(), self.chunk.get_line(self.ip))),
                                _ => return Err(Error::RUNTIME_ERROR("TypeError: Unsupported operand types for `*`".into(), self.chunk.get_line(self.ip)))
                            }
                        }
                    }
                },
                OpCode::REM => {
                    if self.stack.len() < 2 {
                        return Err(Error::RUNTIME_ERROR("IndexError: Stack index out of range".into(), self.chunk.get_line(self.ip)));
                    }
                    let value = binary_op!(self, %)?;
                    self.stack.push(value);
                },
                OpCode::EQUAL => {
                    if self.stack.len() < 2 {
                        return Err(Error::RUNTIME_ERROR("IndexError: Stack index out of range".into(), self.chunk.get_line(self.ip)));
                    }
                    let value = binary_op!(self, ==);
                    self.stack.push(Value::BOOL(value));
                },
                OpCode::GREATER => {
                    if self.stack.len() < 2 {
                        return Err(Error::RUNTIME_ERROR("IndexError: Stack index out of range".into(), self.chunk.get_line(self.ip)));
                    }
                    let value = binary_op!(self, >);
                    self.stack.push(Value::BOOL(value));
                },
                OpCode::LESS => {
                    if self.stack.len() < 2 {
                        return Err(Error::RUNTIME_ERROR("IndexError: Stack index out of range".into(), self.chunk.get_line(self.ip)));
                    }
                    let value = binary_op!(self, <);
                    self.stack.push(Value::BOOL(value));
                },
                OpCode::IF(jaddr) => {
                    if *self.stack.last().unwrap() == Value::BOOL(false) {
                        self.ip = *jaddr;
                    }
                },
                OpCode::IFN(jaddr) => {
                    if *self.stack.last().unwrap() != Value::BOOL(false) {
                        self.ip = *jaddr;
                    }
                },
                OpCode::JMP(jaddr) => {
                    self.ip = *jaddr;
                },
            };
        }
    }
}