use crate::vm::interpret;
use crate::vm::InterpretResult;
use std::env;
use std::io;
use std::fs;
use std::io::Write;

mod chunk;
mod debug;
mod value;
mod vm;
mod compiler;
mod scanner;

fn repl() {
    loop {
        print!("> ");
        io::stdout().flush().expect("fail: flush");
        
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {},
            Err(_) => { return; }
        }
        interpret(line);
    }
}

fn run_file(path: String) {
    let contents = fs::read_to_string(path).expect("fail: read file");
    let result = interpret(contents);
    if result == InterpretResult::CompileError {
        std::process::exit(65);
    }
    if result == InterpretResult::RuntimeError {
        std::process::exit(70);
    }
}

fn main() {
    if env::args().len() == 1 {
        repl();
    } else if env::args().len() == 2 {
        run_file(env::args().nth(1).unwrap());
    } else {
        println!("Usage: rustlox [path]");
    }
}
