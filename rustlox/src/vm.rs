// Purpose: Lox Virtual Machine

use std::collections::HashMap;
use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::value::Value;
use crate::debug::disassemble_instruction;
use crate::compiler::compile;
use crate::object::Obj;
use crate::object::ObjArray;
use crate::object::ObjFunction;
use crate::object::NativeFn;
use std::rc::Rc;
use std::time::Instant;

const DEBUG: bool = false;
const UINT8_COUNT: usize = 256;
const FRAMES_MAX: usize = 64;
const STACK_MAX: usize = FRAMES_MAX * UINT8_COUNT;

#[derive(Debug)]
pub struct VM<'a> {
    stack: [Value; STACK_MAX],
    stack_top: usize,
    obj_array: &'a mut ObjArray,
    globals: HashMap<&'static str, Value>,
    frames: [CallFrame; FRAMES_MAX],
    frame_count: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct CallFrame {
    pub function: *const ObjFunction,
    pub ip: usize,
    pub stack_top: usize,
}

impl CallFrame {
    pub fn chunk(&self) -> &Chunk {
        unsafe { &(*(*self.function).chunk) }
    }
}

impl Default for CallFrame {
    fn default() -> CallFrame {
        CallFrame {
            function: std::ptr::null(),
            ip: 0,
            stack_top: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub fn interpret(source: String) -> InterpretResult {
    let mut obj_array = ObjArray::default();
    let chunk = Rc::new(Chunk::default());
    let func = compile(source, chunk, &mut obj_array);
    if func.is_none() {
        return InterpretResult::CompileError;
    }

    let mut vm = VM {
        stack: [Value::number(0.0); STACK_MAX],
        stack_top: 0,
        obj_array: &mut obj_array,
        globals: HashMap::new(),
        frames: std::array::from_fn(|_| CallFrame::default()),
        frame_count: 0,
    };
    vm.define_native("clock", new_clock_native());
    vm.push(Value::object(func.unwrap() as *const Obj));
    vm.call(&CallFrame::default(), func.unwrap(), 0);
    
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
    
    fn read_byte(&mut self, frame: &mut CallFrame) -> u8 {
        let byte = frame.chunk().code[frame.ip];
        frame.ip = frame.ip + 1;
        return byte;
    }
    
    fn read_short(&mut self, frame: &mut CallFrame) -> u16 {
        let chunk = frame.chunk();
        let short = (chunk.code[frame.ip] as u16) << 8 | chunk.code[frame.ip + 1] as u16;
        frame.ip = frame.ip + 2;
        return short;
    }

    fn read_constant(&mut self, frame: &mut CallFrame) -> Value {
        let byte = self.read_byte(frame) as usize;
        return unsafe { (*(*frame.function).chunk).constants.values[byte] }
    }

    fn runtime_error(&mut self, frame: &CallFrame, message: &str) {
        eprintln!("{}", message);
        self.print_frame(frame);
        for i in (0..self.frame_count - 1).rev() {
            self.print_frame(&self.frames[i]);
        }
    }

    fn print_frame(&self, frame: &CallFrame) {
        let function = unsafe { (*frame.function).name };
        let instruction = frame.ip - 1;
        let line = frame.chunk().lines[instruction];
        eprint!("[line {}] in ", line);
        if function.is_null() {
            eprintln!("script");
        } else {
            eprintln!("{}()", unsafe { (*function).as_str() });
        }
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

    fn call(&mut self, orig_frame: &CallFrame, callee: *const ObjFunction, arg_count: usize) -> bool {
        let arity = unsafe { (*callee).arity };
        if arg_count != arity as usize {
            self.runtime_error(orig_frame, "Wrong number of arguments.");
            return false;
        }
        if self.frame_count == FRAMES_MAX {
            self.runtime_error(orig_frame, "Stack overflow.");
            return false;
        }
        
        let mut frame = &mut self.frames[self.frame_count];
        frame.function = callee;
        frame.ip = 0;
        frame.stack_top = self.stack_top - arg_count - 1;

        self.frame_count += 1;
        return true;
    }

    fn define_native(&mut self, name: &str, function: NativeFn) {
        let val = self.obj_array.copy_string(name);
        self.push(Value::object(val as *const Obj));
        let native = self.obj_array.new_native(function);
        self.push(Value::object(native as *const Obj));
        
        unsafe {
            let n = self.peek(1).as_string();
            let slice = std::slice::from_raw_parts((*n).chars, (*n).len);
            let s = std::str::from_utf8(slice).unwrap();
            self.globals.insert(s, self.peek(0));
        }
        self.pop();
        self.pop();
    }

    fn call_value(&mut self, frame: &CallFrame, callee: Value, arg_count: usize) -> bool {
        if callee.is_function() {
            return self.call(frame, callee.as_function(), arg_count);
        }
        if callee.is_native() {
            let native = callee.as_native();
            let result = unsafe {
                ((*native).function)(arg_count, &self.stack[self.stack_top..self.stack_top+arg_count])
            };
                
            self.stack_top -= arg_count + 1;
            self.push(result);
            return true;
        }

        self.runtime_error(frame, "Can only call functions and classes.");
        return false;
    }

    fn run(&mut self) -> InterpretResult {
        let mut frame = std::mem::take(&mut self.frames[self.frame_count - 1]);
        
        loop {
            if DEBUG {
                print!("          ");
                for i in 0..self.stack_top {
                    print!("[ ");
                    self.stack[i].print();
                    print!(" ]");
                }
                println!();
                
                disassemble_instruction(frame.chunk(), frame.ip);
            }
            
            let instruction = self.read_byte(&mut frame);
            match OpCode::try_from(instruction) {
                Ok(OpCode::Print) => {
                    self.pop().print();
                    println!();
                }
                Ok(OpCode::Pop) => {
                    self.pop();
                }
                Ok(OpCode::DefineGlobal) => {
                    let constant = self.read_constant(&mut frame);
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
                    let constant = self.read_constant(&mut frame);
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
                            self.runtime_error(&mut frame, "Undefined variable.");
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                Ok(OpCode::GetGlobal) => {
                    let constant = self.read_constant(&mut frame);
                    let value = self.globals.get(constant.as_str());
                    match value {
                        Some(v) => {
                            self.push(*v);
                        }
                        None => {
                            self.runtime_error(&mut frame, "Undefined variable.");
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                Ok(OpCode::GetLocal) => {
                    let slot = self.read_byte(&mut frame) as usize;
                    self.push(self.stack[frame.stack_top + slot]);
                }
                Ok(OpCode::SetLocal) => {
                    let slot = self.read_byte(&mut frame) as usize;
                    self.stack[frame.stack_top + slot] = self.peek(0);
                }
                Ok(OpCode::Jump) => {
                    let offset = self.read_short(&mut frame) as usize;
                    frame.ip = frame.ip + offset;
                }
                Ok(OpCode::Loop) => {
                    let offset = self.read_short(&mut frame) as usize;
                    frame.ip = frame.ip - offset;
                }
                Ok(OpCode::JumpIfFalse) => {
                    let offset = self.read_short(&mut frame) as usize;
                    if self.peek(0).is_falsey() {
                        frame.ip = frame.ip + offset;
                    }
                }
                Ok(OpCode::Call) => {
                    let orig_frame = self.frame_count - 1;
                    let arg_count = self.read_byte(&mut frame) as usize;
                    if !self.call_value(&frame, self.peek(arg_count), arg_count) {
                        return InterpretResult::RuntimeError;
                    }
                    self.frames[orig_frame] = frame;
                    frame = std::mem::take(&mut self.frames[self.frame_count - 1]);
                }
                Ok(OpCode::Return) => {
                    let result = self.pop();
                    self.frame_count -= 1;
                    if self.frame_count == 0 {
                        self.pop();
                        return InterpretResult::Ok;
                    }
                    self.stack_top = frame.stack_top;
                    self.push(result);
                    frame = std::mem::take(&mut self.frames[self.frame_count - 1]);
                }
                Ok(OpCode::Constant) => {
                    let constant = self.read_constant(&mut frame);
                    self.push(constant);
                }
                Ok(OpCode::Negate) => {
                    let val = self.peek(0);
                    if !val.is_number() {
                        self.runtime_error(&mut frame, "Operand must be a number.");
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
                        self.runtime_error(&mut frame, "Operands must be two numbers or two strings.");
                        return InterpretResult::RuntimeError;
                    }
                }
                Ok(OpCode::Subtract) => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error(&mut frame, "Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::number(a.as_number() - b.as_number()));
                }
                Ok(OpCode::Multiply) => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error(&mut frame, "Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::number(a.as_number() * b.as_number()));
                }
                Ok(OpCode::Divide) => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error(&mut frame, "Operands must be numbers.");
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
                        self.runtime_error(&mut frame, "Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::bool(a.as_number() > b.as_number()));
                }
                Ok(OpCode::Less) => {
                    if !self.peek(0).is_number() || !self.peek(1).is_number() {
                        self.runtime_error(&mut frame, "Operands must be numbers.");
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

fn new_clock_native() -> Box<dyn Fn(usize, &[Value]) -> Value> {
    let start = Instant::now();
    Box::new(move |_, _| {
        return Value::number(start.elapsed().as_secs_f64())
    })
}
