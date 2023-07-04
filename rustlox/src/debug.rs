// Purpose: Debugging functions for the VM.

use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::value::print_value;

fn simple_instruction(name: &str, offset: usize) -> usize {
    print!("{:16}\n", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    print!("{:16} {:4} '", name, constant);
    print_value(chunk.constants.values[constant as usize]);
    print!("'\n");
    offset + 2
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }
    
    let instruction = chunk.code[offset];
    match OpCode::from_u8(instruction) {
        Some(OpCode::OpReturn) => {
            return simple_instruction("OP_RETURN", offset)
        }
        Some(OpCode::OpConstant) => {
            return constant_instruction("OP_CONSTANT", chunk, offset)
        }
        Some(OpCode::OpNegate) => {
            return simple_instruction("OP_NEGATE", offset)
        }
        Some(OpCode::OpAdd) => {
            return simple_instruction("OP_ADD", offset)
        }
        Some(OpCode::OpSubtract) => {
            return simple_instruction("OP_SUBTRACT", offset)
        }
        Some(OpCode::OpMultiply) => {
            return simple_instruction("OP_MULTIPLY", offset)
        }
        Some(OpCode::OpDivide) => {
            return simple_instruction("OP_DIVIDE", offset)
        }
        _ => {
            print!("Unknown opcode {}\n", instruction);
            return offset + 1
        }
    }
}

#[allow(dead_code)]
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    print!("== {} ==\n", name);
    let mut i = 0;
    while i < chunk.code.len() {
        i = disassemble_instruction(chunk, i);
    }
}
