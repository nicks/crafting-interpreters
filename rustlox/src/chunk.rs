// Purpose: In-memory bytecode representation.

use crate::value::ValueArray;
use crate::value::write_value_array;
use num_enum::TryFromPrimitive;
use num_enum::IntoPrimitive;

#[repr(u8)]
#[derive(Debug, TryFromPrimitive, IntoPrimitive)]
pub enum OpCode {
    Constant = 0,
    Return = 1,
    Negate = 2,
    Add = 3,
    Subtract = 4,
    Multiply = 5,
    Divide = 6,
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
    
    pub fn add_constant(&mut self, value: f64) -> usize {
        write_value_array(&mut self.constants, value);
        self.constants.values.len() - 1
    }
}
