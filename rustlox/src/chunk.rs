// Purpose: In-memory bytecode representation.

use crate::value::ValueArray;
use crate::value::Value;
use num_enum::TryFromPrimitive;
use num_enum::IntoPrimitive;

#[repr(u8)]
#[derive(Debug, TryFromPrimitive, IntoPrimitive)]
pub enum OpCode {
    Constant,
    Return,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Nil,
    True,
    False,
    Not,
    Equal,
    Greater,
    Less,
    Print,
    Pop,
    DefineGlobal,
    GetGlobal,
    SetGlobal,
}
    
#[derive(Debug, Default)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
    pub lines: Vec<i32>
}

impl Chunk {
    pub fn write_chunk(&mut self, code: u8, line: i32) {
        self.code.push(code);
        self.lines.push(line);
    }
    
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);
        self.constants.values.len() - 1
    }
}
