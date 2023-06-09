// Heap-allocated objects

use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Debug;
use std::alloc::Layout;

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
                let slice = std::slice::from_raw_parts((*sp).chars, (*sp).len);
                let s = std::str::from_utf8_unchecked(slice);
                return write!(f, "{}", s);
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ObjType {
    String,
}

#[repr(C)]
pub struct ObjString {
    pub obj: Obj,
    pub len: usize,
    pub chars: *const u8,
}

#[derive(Debug)]
pub struct ObjArray {
    pub objects: *mut Obj,
}

impl ObjArray {
    pub fn default() -> ObjArray {
        ObjArray {
            objects: std::ptr::null_mut(),
        }
    }

    pub fn free_objects(&mut self) {
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
            }
        }
    }

    pub fn write(&mut self, obj: *mut Obj) {
        unsafe {
            (*obj).next = self.objects;
            self.objects = obj;
        }
    }
    
    pub fn take_string(&mut self, s: &str) -> *const ObjString {
        self.copy_string(s)
    }
    
    pub fn copy_string(&mut self, s: &str) -> *const ObjString {
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
        return ptr as *const ObjString;
    }

}
