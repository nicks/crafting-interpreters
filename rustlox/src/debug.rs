// Purpose: Debugging functions for the VM.

use crate::chunk::Chunk;
use crate::chunk::OpCode;

fn simple_instruction(name: &str, offset: usize) -> usize {
    print!("{:16}\n", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    print!("{:16} {:4} '", name, constant);
    chunk.constants.values[constant as usize].print();
    print!("'\n");
    offset + 2
}

fn byte_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let slot = chunk.code[offset + 1];
    print!("{:16} {:4}\n", name, slot);
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
    match OpCode::try_from(instruction) {
        Ok(OpCode::DefineGlobal) => {
            return constant_instruction("OP_DEFINE_GLOBAL", chunk, offset)
        }
        Ok(OpCode::SetGlobal) => {
            return constant_instruction("OP_SET_GLOBAL", chunk, offset)
        }
        Ok(OpCode::GetGlobal) => {
            return constant_instruction("OP_GET_GLOBAL", chunk, offset)
        }
        Ok(OpCode::GetLocal) => {
            return byte_instruction("OP_GET_LOCAL", chunk, offset)
        }
        Ok(OpCode::SetLocal) => {
            return byte_instruction("OP_SET_LOCAL", chunk, offset)
        }
        Ok(OpCode::Pop) => {
            return simple_instruction("OP_POP", offset)
        }
        Ok(OpCode::Print) => {
            return simple_instruction("OP_PRINT", offset)
        }
        Ok(OpCode::Return) => {
            return simple_instruction("OP_RETURN", offset)
        }
        Ok(OpCode::Constant) => {
            return constant_instruction("OP_CONSTANT", chunk, offset)
        }
        Ok(OpCode::Negate) => {
            return simple_instruction("OP_NEGATE", offset)
        }
        Ok(OpCode::Add) => {
            return simple_instruction("OP_ADD", offset)
        }
        Ok(OpCode::Subtract) => {
            return simple_instruction("OP_SUBTRACT", offset)
        }
        Ok(OpCode::Multiply) => {
            return simple_instruction("OP_MULTIPLY", offset)
        }
        Ok(OpCode::Divide) => {
            return simple_instruction("OP_DIVIDE", offset)
        }
        Ok(OpCode::Nil) => {
            return simple_instruction("OP_NIL", offset)
        }
        Ok(OpCode::True) => {
            return simple_instruction("OP_TRUE", offset)
        }
        Ok(OpCode::False) => {
            return simple_instruction("OP_FALSE", offset)
        }
        Ok(OpCode::Not) => {
            return simple_instruction("OP_NOT", offset)
        }
        Ok(OpCode::Equal) => {
            return simple_instruction("OP_EQUAL", offset)
        }
        Ok(OpCode::Greater) => {
            return simple_instruction("OP_GREATER", offset)
        }
        Ok(OpCode::Less) => {
            return simple_instruction("OP_LESS", offset)
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
