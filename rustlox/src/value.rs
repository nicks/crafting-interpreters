// Purpose: In-memory bytecode representation.

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ValueType {
    Bool,
    Nil,
    Number,
}

#[derive(Debug, Copy, Clone)]
pub struct Value {
    pub t: ValueType,
    pub value: f64,
}

impl Value {
    pub fn number(value: f64) -> Value {
        Value {
            t: ValueType::Number,
            value,
        }
    }

    pub fn bool(value: bool) -> Value {
        Value {
            t: ValueType::Bool,
            value: if value { 1.0 } else { 0.0 },
        }
    }

    pub fn nil() -> Value {
        Value {
            t: ValueType::Nil,
            value: 0.0,
        }
    }
    
    pub fn print(&self) {
        match self.t {
            ValueType::Bool => {
                if self.as_bool() {
                    print!("true");
                } else {
                    print!("false");
                }
            }
            ValueType::Nil => print!("nil"),
            ValueType::Number => print!("{}", self.as_number()),
        }
    }

    pub fn is_falsey(&self) -> bool {
        self.is_nil() || (self.is_bool() && !self.as_bool())
    }

    pub fn equals(&self, other: Value) -> bool {
        if self.t != other.t {
            return false;
        }

        match self.t {
            ValueType::Bool => self.as_bool() == other.as_bool(),
            ValueType::Nil => true,
            ValueType::Number => self.as_number() == other.as_number(),
        }
    }
    
    pub fn is_bool(&self) -> bool {
        self.t == ValueType::Bool
    }
    
    pub fn is_nil(&self) -> bool {
        self.t == ValueType::Nil
    }
    
    pub fn is_number(&self) -> bool {
        self.t == ValueType::Number
    }
    
    pub fn as_bool(&self) -> bool {
        self.value != 0.0
    }
    
    pub fn as_number(&self) -> f64 {
        self.value
    }
}
    
#[derive(Debug, Default)]
pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn write(&mut self, value: Value) {
        self.values.push(value);
    }
}
