use mdl_sys as sys;

use std::ffi::{CStr, CString};
use std::path::Path;

use err_derive::Error;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Type {
    pub(crate) ptr: sys::IType,
}

impl Drop for Type {
    fn drop(&mut self) {
        unsafe { sys::IType_release(self.ptr) };
    }
}

impl Clone for Type {
    fn clone(&self) -> Type {
        unsafe { sys::IType_retain(self.ptr) };
        Type { ptr: self.ptr }
    }
}

impl Type {}

pub struct TypeList {
    pub(crate) ptr: sys::ITypeList,
}

impl Drop for TypeList {
    fn drop(&mut self) {
        unsafe { sys::IType_list_release(self.ptr) };
    }
}

impl Clone for TypeList {
    fn clone(&self) -> TypeList {
        unsafe { sys::IType_list_retain(self.ptr) };
        TypeList { ptr: self.ptr }
    }
}

impl TypeList {
    pub fn get_size(&self) -> usize {
        unsafe { sys::IType_list_get_size(self.ptr) }
    }

    pub fn get_index(&self, name: &str) -> Option<usize> {
        let cname = CString::new(name).unwrap();
        let index = unsafe { sys::IType_list_get_index(self.ptr, cname.as_ptr()) };
        if index == std::usize::MAX {
            None
        } else {
            Some(index)
        }
    }

    pub fn get_name(&self, index: usize) -> Option<String> {
        let cname = unsafe { sys::IType_list_get_name(self.ptr, index) };
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

    pub fn get_type(&self, index: usize) -> Option<Type> {
        let ptr = unsafe { sys::IType_list_get_type(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(Type { ptr })
        }
    }

    pub fn iter(&self) -> TypeListIterator {
        TypeListIterator {
            type_list: self,
            current: 0,
            count: self.get_size(),
        }
    }
}

pub struct TypeListIterator<'a> {
    type_list: &'a TypeList,
    current: usize,
    count: usize,
}

impl<'a> Iterator for TypeListIterator<'a> {
    type Item = Type;
    fn next(&mut self) -> Option<Type> {
        if self.current < self.count {
            self.current += 1;
            self.type_list.get_type(self.current - 1)
        } else {
            None
        }
    }
}

pub struct TypeFactory {
    pub(crate) ptr: sys::ITypeFactory,
}

impl TypeFactory {
    pub fn dump(&self, t: &Type, depth: usize) -> Option<String> {
        let ptr = unsafe { sys::IType_factory_dump(self.ptr, t.ptr, depth) };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string() })
        }
    }
}

pub enum Error {}
