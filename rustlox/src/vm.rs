// Purpose: Lox Virtual Machine

use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::value::Value;
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
        stack: [Value::number(0.0); STACK_MAX],
        stack_top: 0,
    };
    return vm.run();
}

impl VM<'_> {
    fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top = self.stack_top + 1;
    }

    fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack_top - 1 - distance]
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

    fn read_constant(&mut self) -> Value {
        let byte = self.read_byte() as usize;
        self.chunk.constants.values[byte]
    }

    fn runtime_error(&self, message: &str) {
        eprintln!("{}", message);
        let instruction = self.ip - 1;
        let line = self.chunk.lines[instruction];
        eprintln!("[line {}] in script", line);
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            if DEBUG {
                print!("          ");
                for i in 0..self.stack_top {
                    print!("[ ");
                    self.stack[i].print();
                    print!(" ]");
                }
                println!();
                
                disassemble_instruction(self.chunk, self.ip);
            }
            
            let instruction = self.read_byte();
            match OpCode::try_from(instruction) {
                Ok(OpCode::Return) => {
                    self.pop().print();
                    println!();
                    return InterpretResult::Ok;
                }
                Ok(OpCode::Constant) => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                Ok(OpCode::Negate) => {
                    let val = self.peek(0);
                    if !val.is_number() {
                        self.runtime_error("Operand must be a number.");
                        return InterpretResult::RuntimeError;
                    }
                    let a = self.pop();
                    self.push(Value::number(-a.as_number()));
                }
                Ok(OpCode::Add) => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::number(a.as_number() + b.as_number()));
                }
                Ok(OpCode::Subtract) => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::number(a.as_number() - b.as_number()));
                }
                Ok(OpCode::Multiply) => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::number(a.as_number() * b.as_number()));
                }
                Ok(OpCode::Divide) => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::number(a.as_number() / b.as_number()));
                }
                Ok(OpCode::Nil) => self.push(Value::nil()),
                Ok(OpCode::True) => self.push(Value::bool(true)),
                Ok(OpCode::False) => self.push(Value::bool(false)),
                Ok(OpCode::Equal) => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::bool(a.equals(b)));
                }
                Ok(OpCode::Not) => {
                    let val = self.pop();
                    self.push(Value::bool(val.is_falsey()));
                }
                Ok(OpCode::Greater) => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::bool(a.as_number() > b.as_number()));
                }
                Ok(OpCode::Less) => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error("Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::bool(a.as_number() < b.as_number()));
                }
                _ => {
                    println!("Unknown opcode {}", instruction);
                return InterpretResult::RuntimeError;
                }
            }
        }
    }
}
