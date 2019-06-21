use mdl_sys as sys;

use std::ffi::{CStr, CString};


type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Value {
    pub(crate) ptr: sys::IValue,
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe { sys::IValue_release(self.ptr) };
    }
}

impl Clone for Value {
    fn clone(&self) -> Value {
        unsafe { sys::IValue_retain(self.ptr) };
        Value { ptr: self.ptr }
    }
}

impl Value {}

pub struct ValueList {
    pub(crate) ptr: sys::IValueList,
}

impl Drop for ValueList {
    fn drop(&mut self) {
        unsafe { sys::IValue_list_release(self.ptr) };
    }
}

impl Clone for ValueList {
    fn clone(&self) -> ValueList {
        unsafe { sys::IValue_list_retain(self.ptr) };
        ValueList { ptr: self.ptr }
    }
}

impl ValueList {
    pub fn get_size(&self) -> usize {
        unsafe { sys::IValue_list_get_size(self.ptr) }
    }

    pub fn get_index(&self, name: &str) -> Option<usize> {
        let cname = CString::new(name).unwrap();
        let index =
            unsafe { sys::IValue_list_get_index(self.ptr, cname.as_ptr()) };
        if index == std::usize::MAX {
            None
        } else {
            Some(index)
        }
    }

    pub fn get_name(&self, index: usize) -> Option<String> {
        let cname = unsafe { sys::IValue_list_get_name(self.ptr, index) };
        if cname.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(cname)
                    .to_string_lossy()
                    .to_owned()
                    .to_string()
            })
        }
    }

    pub fn get_value(&self, index: usize) -> Option<Value> {
        let ptr = unsafe { sys::IValue_list_get_value(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(Value { ptr })
        }
    }

    pub fn iter(&self) -> ValueListIterator {
        ValueListIterator {
            value_list: self,
            current: 0,
            count: self.get_size(),
        }
    }
}

pub struct ValueListIterator<'a> {
    value_list: &'a ValueList,
    current: usize,
    count: usize,
}

impl<'a> Iterator for ValueListIterator<'a> {
    type Item = Value;
    fn next(&mut self) -> Option<Value> {
        if self.current < self.count {
            self.current += 1;
            self.value_list.get_value(self.current - 1)
        } else {
            None
        }
    }
}

pub struct ValueFactory {
    pub(crate) ptr: sys::IValueFactory,
}

impl ValueFactory {
    pub fn dump(
        &self,
        t: &Value,
        name: Option<&str>,
        depth: usize,
    ) -> Option<String> {
        let ptr = if let Some(name) = name {
            let name = CString::new(name).unwrap();
            unsafe {
                sys::IValue_factory_dump(self.ptr, t.ptr, name.as_ptr(), depth)
            }
        } else {
            unsafe {
                sys::IValue_factory_dump(
                    self.ptr,
                    t.ptr,
                    std::ptr::null(),
                    depth,
                )
            }
        };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }
}

pub enum Error {}
