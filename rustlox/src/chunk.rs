// Purpose: In-memory bytecode representation.

use crate::value::ValueArray;
use crate::value::write_value_array;

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    OpConstant = 0,
    OpReturn = 1,
}

impl OpCode {
    pub fn to_u8(self) -> u8 {
        self as u8
    }
    
    pub fn from_u8(byte: u8) -> Option<OpCode> {
        match byte {
            0 => Some(OpCode::OpConstant),
            1 => Some(OpCode::OpReturn),
            _ => None,
        }
    }
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
