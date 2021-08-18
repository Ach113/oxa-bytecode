use crate::value::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    RETURN,
    CONSTANT(usize),
    //literal
    TRUE,
    FALSE,
    NIL,
    // unary ops
    NEGATE,
    BANG,
    // binary ops
    ADD,
    SUB,
    MUL,
    DIV,
    REM,
    OR,
    AND,
}

impl OpCode {
    pub fn dissassemble_instruction(&self, chunk: &Chunk, offset: usize) -> usize {
        print!("{:0>4}  ", offset);
        if offset > 0 && chunk.get_line(offset) == chunk.get_line(offset - 1) {
          print!("| ");
        } else {
          print!("{} ", chunk.get_line(offset));
        }
        match self {
            OpCode::RETURN => {
                println!("{:?}", self);
                return offset + 1;
            },
            OpCode::CONSTANT(addr) => {
                println!("{:?} at {:0>4?}", chunk.values[*addr], addr);
                return offset + 1;
            },
            _ => {
                println!("{:?}", self);
                return offset + 1;
            },
        }
    }
}

#[derive(Default)]
pub struct Chunk {
    code: Vec<OpCode>, // each instruction is byte long
    values: Vec<Value>, // immediate types
    lines: Vec<usize>, // index: line no, value: no of instructions on that line
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {code: vec![], values: vec![], lines: vec![]}
    }

    pub fn read_instruction(&self, ip: &mut usize) -> &OpCode {
        *ip += 1;
        &self.code[*ip - 1]
    }

    pub fn read_value(&self, addr: usize) -> Value {
        self.values[addr].clone()
    }

    pub fn write_chunk(&mut self, byte: OpCode, line: usize) {
        self.code.push(byte);
        self.add_line(line);
    }

    fn add_line(&mut self, line: usize) {
        let n = self.lines.len();
        if line == n {
            self.lines[n-1] += 1;
        } else {
            while line > self.lines.len() + 1 {
                self.lines.push(0);
            }
            self.lines.push(1);
        }
    }

    pub fn get_line(&self, mut offset: usize) -> usize {
        for (i, line) in self.lines.iter().enumerate() {
            for _ in 0..(*line) {
                if offset == 0 {
                    return i + 1;
                }
                offset -= 1;
            }
        }
        return 0;
    }

    pub fn write_value(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }

    /* displays contents of the chunk
    pub fn dissassemble_chunk(&self) {
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.code[offset].dissassemble_instruction(self, offset);
        }
    } */
}