use crate::chunk::Chunk;
use crate::chunk::write_chunk;
use crate::chunk::add_constant;
use crate::chunk::OpCode;
use crate::vm::interpret;

mod chunk;
mod debug;
mod value;
mod vm;

fn main() {
    let mut chunk = Chunk::default();
    let constant = add_constant(&mut chunk, 1.2);
    write_chunk(&mut chunk, OpCode::OpConstant.to_u8(), 123);
    write_chunk(&mut chunk, constant as u8, 123);
    let constant2 = add_constant(&mut chunk, 3.4);
    write_chunk(&mut chunk, OpCode::OpConstant.to_u8(), 123);
    write_chunk(&mut chunk, constant2 as u8, 123);

    write_chunk(&mut chunk, OpCode::OpAdd.to_u8(), 123);

    let constant3 = add_constant(&mut chunk, 5.6);
    write_chunk(&mut chunk, OpCode::OpConstant.to_u8(), 123);
    write_chunk(&mut chunk, constant3 as u8, 123);

    write_chunk(&mut chunk, OpCode::OpDivide.to_u8(), 123);
    write_chunk(&mut chunk, OpCode::OpNegate.to_u8(), 123);
    write_chunk(&mut chunk, OpCode::OpReturn.to_u8(), 123);
    let result = interpret(&chunk);
    if result == vm::InterpretResult::Ok {
        println!("Ok");
    } else {
        println!("Error");
    }
}
