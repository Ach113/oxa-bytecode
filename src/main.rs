use std::env;
use std::io::Write;

mod chunk;
mod value;
mod vm;
mod token;
mod compiler;
mod scanner;

use vm::VM;
use compiler::Compiler;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Error {
    COMPILE_ERROR(String, usize),
    RUNTIME_ERROR(String, usize),
    DIVIDE_BY_ZERO,
    FILE_NOT_FOUND,
    IO_ERROR,
    SIGNAL
}

fn interpret(code: String) -> Result<(), Error> {
    // declare virtual machine which will interpret the bytecode
    let mut vm = VM::default();
    // instantiate the compiler
    let mut compiler = Compiler::new(code);
    // compile the source code into bytecode
    if let Err(e) = compiler.compile() {
        return Err(e);
    }
    // set VM with chunk of bytecode
    vm.set_chunk(compiler.chunk);
    // run the VM
    vm.execute(true)
}

// reads text from source file and runs it
fn runfile(filename: &str) -> Result<(), Error> {
    match std::fs::read_to_string(filename) {
        Ok(code) => {
            interpret(code)
        },
        Err(_) => {
            println!("FileNotFound: file `{}` could not be found", filename);
            Err(Error::FILE_NOT_FOUND)
        }
    }
    
}

fn repl() -> Result<(), Error> {
    loop {
        print!(">> ");
        // necessary due to line-buffering of stdout
        match std::io::stdout().flush() {
            Ok(_) => {},
            Err(_) => return Err(Error::IO_ERROR)
        };
        let mut instruction = String::new();
        match std::io::stdin().read_line(&mut instruction) {
            Ok(_) => {},
            Err(_) => return Err(Error::IO_ERROR)
        };
        if !instruction.trim().is_empty() {
            interpret(instruction)?;
        }
    }
}

fn main() -> Result<(), Error> {
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
}
