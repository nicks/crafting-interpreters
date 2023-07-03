// Purpose: In-memory bytecode representation.

pub type Value = f64;
    
#[derive(Debug, Default)]
pub struct ValueArray {
    pub values: Vec<Value>,
}

pub fn write_value_array(array: &mut ValueArray, value: Value) {
    array.values.push(value);
}

pub fn print_value(value: Value) {
    print!("{}", value);
}
