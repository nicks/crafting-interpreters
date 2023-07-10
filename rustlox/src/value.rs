// Purpose: In-memory bytecode representation.

use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Debug;
use crate::object::ObjType;
use crate::object::Obj;
use crate::object::ObjString;
use crate::object::obj_fmt;

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ValueType {
    Bool,
    Nil,
    Number,
    Obj,
}

#[derive(Copy, Clone)]
pub struct Value {
    pub t: ValueType,
    pub as_: As,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union As {
    pub boolean: bool,
    pub number: f64,
    pub obj: *const Obj,
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.t {
            ValueType::Bool => {
                if self.as_bool() {
                    return write!(f, "true");
                } else {
                    return write!(f, "false");
                }
            }
            ValueType::Nil => write!(f, "nil"),
            ValueType::Number => write!(f, "{}", self.as_number()),
            ValueType::Obj => obj_fmt(self.as_object(), f),
        }
    }
}

impl Value {
    pub fn number(value: f64) -> Value {
        Value {
            t: ValueType::Number,
            as_: As{number: value},
        }
    }

    pub fn bool(value: bool) -> Value {
        Value {
            t: ValueType::Bool,
            as_: As{boolean: value},
        }
    }

    pub fn nil() -> Value {
        Value {
            t: ValueType::Nil,
            as_: As{boolean: false},
        }
    }

    pub fn object(value: *const Obj) -> Value {
        Value {
            t: ValueType::Obj,
            as_: As{obj: value},
        }
    }
    
    pub fn print(&self) {
        print!("{:?}", self);
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
            ValueType::Obj => {
                let a = self.as_str();
                let b = other.as_str();
                return a == b;
            }
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

    pub fn is_object(&self) -> bool {
        self.t == ValueType::Obj
    }

    pub fn is_string(&self) -> bool {
        unsafe {
            self.is_object() && (*self.as_object()).t == ObjType::String
        }
    }
    
    pub fn as_bool(&self) -> bool {
        unsafe {
            self.as_.boolean
        }
    }
    
    pub fn as_number(&self) -> f64 {
        unsafe {
            self.as_.number
        }
    }

    pub fn as_object(&self) -> *const Obj {
        unsafe {
            self.as_.obj
        }
    }

    pub fn as_string(&self) -> *const ObjString {
        unsafe {
            self.as_.obj as *const ObjString
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let obj_string = self.as_string();
            let slice = std::slice::from_raw_parts((*obj_string).chars, (*obj_string).len);
            return std::str::from_utf8(slice).unwrap();
        }
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
