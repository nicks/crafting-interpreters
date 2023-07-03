use crate::chunk::Chunk;
use crate::chunk::write_chunk;
use crate::chunk::add_constant;
use crate::chunk::OpCode;
use crate::debug::disassemble_chunk;


mod chunk;
mod debug;
mod value;

fn main() {
    let mut chunk = Chunk::default();
    let constant = add_constant(&mut chunk, 1.2);
    write_chunk(&mut chunk, OpCode::OpConstant.to_u8(), 123);
    write_chunk(&mut chunk, constant as u8, 123);
    
    write_chunk(&mut chunk, OpCode::OpReturn.to_u8(), 123);
    disassemble_chunk(&chunk, "test chunk");
}
