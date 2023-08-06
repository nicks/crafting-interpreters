// Purpose: Lox Virtual Machine

use std::collections::HashMap;
use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::value::Value;
use crate::debug::disassemble_instruction;
use crate::compiler::compile;
use crate::object::Obj;
use crate::object::ObjArray;

const DEBUG: bool = false;
const STACK_MAX: usize = 256;

#[derive(Debug)]
pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: [Value; STACK_MAX],
    stack_top: usize,
    obj_array: &'a mut ObjArray,
    globals: HashMap<&'static str, Value>,
}

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub fn interpret(source: String) -> InterpretResult {
    let mut obj_array = ObjArray::default();
    let mut chunk = Chunk::default();
    if !compile(source, &mut chunk, &mut obj_array) {
        return InterpretResult::CompileError;
    }

    let mut vm = VM {
        chunk: &chunk,
        ip: 0,
        stack: [Value::number(0.0); STACK_MAX],
        stack_top: 0,
        obj_array: &mut obj_array,
        globals: HashMap::new(),
    };
    let result = vm.run();
    vm.globals.clear();
    vm.obj_array.free_objects();
    return result;
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

    fn concatenate(&mut self) {
        let bv = self.pop();
        let av = self.pop();
        let b = bv.as_str();
        let a = av.as_str();

        // TODO(nicks): Could avoid copy here.
        let mut result = String::from(a);
        result.push_str(b);

        let val = self.obj_array.copy_string(result.as_str());
        self.push(Value::object(val as *const Obj));
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
                Ok(OpCode::Print) => {
                    self.peek(0).print();
                    println!();
                }
                Ok(OpCode::Pop) => {
                    self.pop();
                }
                Ok(OpCode::DefineGlobal) => {
                    let constant = self.read_constant();
                    let value = self.peek(0);
                    unsafe {
                        let name = constant.as_string();
                        let slice = std::slice::from_raw_parts((*name).chars, (*name).len);
                        let s = std::str::from_utf8(slice).unwrap();
                        self.globals.insert(s, value);
                    }
                    self.pop();
                }
                Ok(OpCode::SetGlobal) => {
                    let constant = self.read_constant();
                    let value = self.peek(0);
                    match self.globals.get(constant.as_str()) {
                        Some(_) => {
                            unsafe {
                                let name = constant.as_string();
                                let slice = std::slice::from_raw_parts((*name).chars, (*name).len);
                                let s = std::str::from_utf8(slice).unwrap();
                                self.globals.insert(s, value);
                            }
                        }
                        None => {
                            self.runtime_error("Undefined variable.");
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                Ok(OpCode::GetGlobal) => {
                    let constant = self.read_constant();
                    let value = self.globals.get(constant.as_str());
                    match value {
                        Some(v) => {
                            self.push(*v);
                        }
                        None => {
                            self.runtime_error("Undefined variable.");
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                Ok(OpCode::GetLocal) => {
                    let slot = self.read_byte() as usize;
                    self.push(self.stack[slot]);
                }
                Ok(OpCode::SetLocal) => {
                    let slot = self.read_byte() as usize;
                    self.stack[slot] = self.peek(0);
                }
                Ok(OpCode::Return) => {
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
                    if self.peek(0).is_string() && self.peek(1).is_string() {
                        self.concatenate();
                    } else if self.peek(0).is_number() && self.peek(1).is_number() {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(Value::number(a.as_number() + b.as_number()));
                    } else {
                        self.runtime_error("Operands must be two numbers or two strings.");
                        return InterpretResult::RuntimeError;
                    }
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
