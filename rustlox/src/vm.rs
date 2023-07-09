// Purpose: Lox Virtual Machine

use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::value::Value;
use crate::value::print_value;
use crate::debug::disassemble_instruction;
use crate::compiler::compile;

const DEBUG: bool = false;
const STACK_MAX: usize = 256;

#[derive(Debug)]
pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: [Value; STACK_MAX],
    stack_top: usize,
}

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub fn interpret(source: String) -> InterpretResult {
    let mut chunk = Chunk::default();
    if !compile(source, &mut chunk) {
        return InterpretResult::CompileError;
    }

    let mut vm = VM {
        chunk: &chunk,
        ip: 0,
        stack: [0.0; STACK_MAX],
        stack_top: 0,
    };
    return vm.run();
}

impl VM<'_> {
    fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top = self.stack_top + 1;
    }

    fn pop(&mut self) -> Value {
        self.stack_top = self.stack_top - 1;
        self.stack[self.stack_top]
    }
    
    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip = self.ip + 1;
        return byte;
    }

    fn read_constant(&mut self) -> f64 {
        let byte = self.read_byte() as usize;
        self.chunk.constants.values[byte]
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            if DEBUG {
                print!("          ");
                for i in 0..self.stack_top {
                    print!("[ ");
                    print_value(self.stack[i]);
                    print!(" ]");
                }
                println!();
                
                disassemble_instruction(self.chunk, self.ip);
            }
            
            let instruction = self.read_byte();
            match OpCode::try_from(instruction) {
                Ok(OpCode::Return) => {
                    print_value(self.pop());
                    println!();
                    return InterpretResult::Ok;
                }
                Ok(OpCode::Constant) => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                Ok(OpCode::Negate) => {
                    let val = -self.pop();
                    self.push(val);
                }
                Ok(OpCode::Add) => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a + b);
                }
                Ok(OpCode::Subtract) => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a - b);
                }
                Ok(OpCode::Multiply) => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a * b);
                }
                Ok(OpCode::Divide) => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a / b);
                }
                _ => {
                    println!("Unknown opcode {}", instruction);
                return InterpretResult::RuntimeError;
                }
            }
        }
    }
}
