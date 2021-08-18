use crate::chunk::{Chunk, OpCode};
use crate::Error;
use crate::value::Value;

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

    pub fn execute(&mut self, debug: bool) -> Result<(), Error> {
        if debug {
            println!("------------------------------");
        }
        loop {
            let instruction = self.chunk.read_instruction(&mut self.ip);
            if debug {
                instruction.dissassemble_instruction(&self.chunk, self.ip - 1);
            }
            match instruction {
                OpCode::RETURN => {
                    if debug {
                        println!("------------------------------");
                    }
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
                        return Err(Error::RUNTIME_ERROR("IndexError: Stack index out of range".into(), self.chunk.get_line(self.ip)));
                    }
                    let value = -self.stack[n - 1].clone();
                    if let Ok(x) = value {
                        self.stack[n - 1] = x.clone();
                    } else {
                        return Err(Error::RUNTIME_ERROR("TypeError: Unsporrted operand types for `-`".into(), self.chunk.get_line(self.ip)));
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
                        _ => return Err(Error::RUNTIME_ERROR("TypeError: Unsporrted operand types for `!`".into(), self.chunk.get_line(self.ip)))
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
                        return Err(Error::RUNTIME_ERROR("TypeError: Unsporrted operand types for `+`".into(), self.chunk.get_line(self.ip)));
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
                        return Err(Error::RUNTIME_ERROR("TypeError: Unsporrted operand types for `or`".into(), self.chunk.get_line(self.ip)));
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
                        return Err(Error::RUNTIME_ERROR("TypeError: Unsporrted operand types for `and`".into(), self.chunk.get_line(self.ip)));
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
                        return Err(Error::RUNTIME_ERROR("TypeError: Unsporrted operand types for `-`".into(), self.chunk.get_line(self.ip)));
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
                        return Err(Error::RUNTIME_ERROR("TypeError: Unsporrted operand types for `*`".into(), self.chunk.get_line(self.ip)));
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
                                _ => return Err(Error::RUNTIME_ERROR("TypeError: Unsporrted operand types for `*`".into(), self.chunk.get_line(self.ip)))
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
            };
        }
    }
}