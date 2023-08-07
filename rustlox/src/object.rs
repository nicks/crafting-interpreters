// Heap-allocated objects

use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Debug;
use std::alloc::Layout;
use std::collections::HashMap;
use std::str;
use std::slice;
use std::rc::Rc;
use crate::chunk::Chunk;
use crate::value::Value;

#[repr(C)]
pub struct Obj {
    pub t: ObjType,
    pub next: *mut Obj,
}

pub fn obj_fmt(obj: *const Obj, f: &mut Formatter) -> Result {
    unsafe {
        match (*obj).t {
            ObjType::String => {
                let sp = obj as *const ObjString;
                let slice = slice::from_raw_parts((*sp).chars, (*sp).len);
                let s = str::from_utf8_unchecked(slice);
                return write!(f, "{}", s);
            }
            ObjType::Function => {
                let fp = obj as *const ObjFunction;
                if (*fp).name.is_null() {
                    return write!(f, "<script>");
                }
                let slice = slice::from_raw_parts((*(*fp).name).chars, (*(*fp).name).len);
                let s = str::from_utf8_unchecked(slice);
                return write!(f, "<fn {}>", s);
            }
            ObjType::Native => {
                return write!(f, "<native fn>");
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ObjType {
    String,
    Function,
    Native,
}

#[repr(C)]
pub struct ObjString {
    pub obj: Obj,
    pub len: usize,
    pub chars: *const u8,
}

impl ObjString {
    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.chars, self.len);
            return std::str::from_utf8(slice).unwrap();
        }
    }
}

#[repr(C)]
pub struct ObjFunction {
    pub obj: Obj,
    pub arity: u8,
    pub chunk: Rc<Chunk>,
    pub name: *const ObjString,
}

pub type NativeFn = Box<dyn Fn(usize, &[Value]) -> Value>;

#[repr(C)]
pub struct ObjNative {
    pub obj: Obj,
    pub function: NativeFn,
}

#[derive(Debug)]
pub struct ObjArray {
    pub objects: *mut Obj,
    pub strings: HashMap<&'static str, *const ObjString>,
}

impl ObjArray {
    pub fn default() -> ObjArray {
        ObjArray {
            objects: std::ptr::null_mut(),
            strings: HashMap::new(),
        }
    }

    pub fn free_objects(&mut self) {
        self.strings.clear();
        
        let mut obj = self.objects;
        while !obj.is_null() {
            let next = unsafe { (*obj).next };
            self.free_object(obj);
            obj = next;
        }
        self.objects = std::ptr::null_mut();
    }

    pub fn free_object(&mut self, obj: *mut Obj) {
        unsafe {
            match (*obj).t {
                ObjType::String => {
                    let sp = obj as *mut ObjString;
                    let heap_chars_layout = Layout::array::<u8>((*sp).len + 1).unwrap();
                    std::alloc::dealloc((*sp).chars as *mut u8, heap_chars_layout);
                    std::alloc::dealloc(sp as *mut u8, Layout::new::<ObjString>());
                }
                ObjType::Function => {
                    let fp = obj as *mut ObjFunction;
                    drop(&(*fp).chunk);
                    std::alloc::dealloc(fp as *mut u8, Layout::new::<ObjFunction>());
                }
                ObjType::Native => {
                    let fp = obj as *mut ObjNative;
                    std::alloc::dealloc(fp as *mut u8, Layout::new::<ObjNative>());
                }
            }
        }
    }

    pub fn write(&mut self, obj: *mut Obj) {
        unsafe {
            (*obj).next = self.objects;
            self.objects = obj;
        }
    }

    pub fn new_native(&mut self, function: NativeFn) -> *mut ObjNative {
        let layout = Layout::new::<ObjNative>();
        let ptr = unsafe { std::alloc::alloc(layout) } as *mut ObjNative;
        if ptr.is_null() {
            panic!("allocate native: out of memory");
        }
        unsafe {
            ptr.write(ObjNative {
                obj: Obj { t: ObjType::Native, next: std::ptr::null_mut() },
                function: Box::new(function),
            });
        }
        self.write(ptr as *mut Obj);
        return ptr;
    }

    pub fn new_function(&mut self, chunk: Rc<Chunk>) -> *mut ObjFunction {
        let layout = Layout::new::<ObjFunction>();
        let ptr = unsafe { std::alloc::alloc(layout) } as *mut ObjFunction;
        if ptr.is_null() {
            panic!("allocate function: out of memory");
        }
        unsafe {
            ptr.write(ObjFunction {
                obj: Obj { t: ObjType::Function, next: std::ptr::null_mut() },
                arity: 0,
                chunk: chunk,
                name: std::ptr::null_mut(),
            });
        }
        self.write(ptr as *mut Obj);
        return ptr;
    }
    
    pub fn copy_string(&mut self, s: &str) -> *const ObjString {
        let interned = self.strings.get(s);
        if interned.is_some() {
            return (*interned.unwrap()) as *const ObjString;
        }
        
        let len = s.len();
        let heap_chars_layout = Layout::array::<u8>(len + 1).unwrap();
        let heap_chars_ptr = unsafe { std::alloc::alloc(heap_chars_layout) };
        if heap_chars_ptr.is_null() {
            panic!("allocate string: out of memory");
        }
        unsafe {
            std::ptr::copy(s.as_ptr(), heap_chars_ptr, len);
            heap_chars_ptr.add(len).write(0);
        }
        return self.allocate_string(heap_chars_ptr, len);
    }
    
    fn allocate_string(&mut self, chars: *const u8, len: usize) -> *const ObjString {
        let layout = Layout::new::<ObjString>();
        let ptr = unsafe { std::alloc::alloc(layout) } as *mut ObjString;
        if ptr.is_null() {
            panic!("allocate string: out of memory");
        }
        unsafe {
            ptr.write(ObjString {
                obj: Obj { t: ObjType::String, next: std::ptr::null_mut() },
                len: len,
                chars: chars,
            });
        }
        self.write(ptr as *mut Obj);

        let result = ptr as *const ObjString;
        unsafe {
            let slice = std::slice::from_raw_parts(chars, len);
            let s = std::str::from_utf8(slice).unwrap();
            self.strings.insert(&s, result);
        }
        return ptr;
    }

}

