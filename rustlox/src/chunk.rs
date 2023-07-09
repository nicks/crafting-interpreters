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

pub fn write_chunk(chunk: &mut Chunk, code: u8, line: i32) {
    chunk.code.push(code);
    chunk.lines.push(line);
}

pub fn add_constant(chunk: &mut Chunk, value: f64) -> usize {
    write_value_array(&mut chunk.constants, value);
    chunk.constants.values.len() - 1
}
