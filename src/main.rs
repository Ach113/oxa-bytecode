<<<<<<< HEAD
use std::env;
use std::io::Write;

mod chunk;
mod value;
mod vm;
mod token;
mod scanner;

use chunk::*;
use value::*;
use scanner::Scanner;
=======
mod chunk;
mod value;
mod vm;

use chunk::*;
use value::*;
>>>>>>> 0953e4fa7fc7f10f2b4f2fe5c6d092c00cc71745

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Error {
    STRING(String),
    COMPILE_ERROR,
    RUNTIME_ERROR,
}

<<<<<<< HEAD
// function used to report errors to the user
fn error(error: &str, message: &str, line: usize) {
    println!("{} at line {}: {}", error, line, message);
}

fn run(code: String) -> Result<(), Error> {
    let mut scanner = Scanner::new(code);
    scanner.scan_tokens()?;

    let mut line = 0;
    for token in scanner.tokens {
        if token.line != line {
            print!("{} ", token.line);
            line = token.line;
        } else {
            print!("| ");
        }
        println!("{:?} '{}'", token.t, token.lexeme); 
    }
    Ok(())
}

// reads text from source file and runs it
fn runfile(filename: &str) -> Result<(), Error> {
    match std::fs::read_to_string(filename) {
        Ok(code) => {
            run(code)
        },
        Err(_) => {
            println!("FileNotFound: file `{}` could not be found", filename);
            Err(Error::COMPILE_ERROR)
        }
    }
    
}

fn repl() -> Result<(), Error> {
    loop {
        print!(">> ");
        // necessary due to line-buffering of stdout
        match std::io::stdout().flush() {
            Ok(_) => {},
            Err(_) => return Err(Error::COMPILE_ERROR)
        };
        let mut instruction = String::new();
        match std::io::stdin().read_line(&mut instruction) {
            Ok(_) => {},
            Err(_) => return Err(Error::COMPILE_ERROR)
        };
        if !instruction.trim().is_empty() {
            run(instruction)?;
        }
    }
}

fn main() -> Result<(), Error> {
    // declare virtual machine which will interpret the bytecode
    let mut vm = vm::VM::default();
    
    let argv: Vec<_> = env::args().collect();
    let argc = argv.len();

    if argc > 2 {
        println!("Usage: oxa [filename]");
        std::process::exit(64); 
    } else if argc == 2 {
        runfile(&argv[1])
    } else {
        repl()
    }
=======
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
>>>>>>> 0953e4fa7fc7f10f2b4f2fe5c6d092c00cc71745
}
