mod chunk;
mod value;
mod vm;

use chunk::*;
use value::*;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Error {
    STRING(String),
    COMPILE_ERROR,
    RUNTIME_ERROR,
}

fn main() -> Result<(), Error> {
    // declare virtual machine which will interpret the bytecode
    let mut vm = vm::VM::default();
    // declare chunk of bytecode, which contains executable instructions
    let mut chunk = Chunk::new();

    // write to code chunk
    // add constant
    let constant = Value::FLOAT(10.0);
    let address = chunk.write_value(constant); // write value to the value array
    chunk.write_chunk(OpCode::CONSTANT(address), 1);

    let constant = Value::FLOAT(1.0);
    let address = chunk.write_value(constant); // write value to the value array
    chunk.write_chunk(OpCode::CONSTANT(address), 1);
    // add
    chunk.write_chunk(OpCode::ADD, 1);

    let constant = Value::FLOAT(-1.0);
    let address = chunk.write_value(constant); // write value to the value array
    chunk.write_chunk(OpCode::CONSTANT(address), 2);

    // multiply
    chunk.write_chunk(OpCode::MUL, 2);
    // negate
    chunk.write_chunk(OpCode::NEGATE, 2);

    chunk.write_chunk(OpCode::RETURN, 3);
    // display chunk contents
    chunk.dissassemble_chunk();
    println!("");
    
    vm.set_chunk(chunk);
    vm.interpret(false)
}
