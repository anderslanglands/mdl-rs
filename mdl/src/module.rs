use crate::{base::Interface, itype::TypeList, value::ValueList};
use mdl_sys as sys;

use std::ffi::CStr;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Module {
    pub(crate) ptr: sys::IModule,
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe { sys::IModule_release(self.ptr) };
    }
}

impl Clone for Module {
    fn clone(&self) -> Module {
        unsafe {
            sys::IModule_retain(self.ptr);
        }
        Module { ptr: self.ptr }
    }
}

impl Module {
    pub fn get_filename(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::IModule_get_filename(self.ptr))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }

    pub fn get_mdl_name(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::IModule_get_mdl_name(self.ptr))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }

    pub fn get_import_count(&self) -> usize {
        unsafe { sys::IModule_get_import_count(self.ptr) }
    }

    pub fn get_import(&self, index: usize) -> String {
        unsafe {
            CStr::from_ptr(sys::IModule_get_import(self.ptr, index))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }

    pub fn imports(&self) -> ImportIterator {
        ImportIterator {
            module: self,
            current: 0,
            count: self.get_import_count(),
        }
    }

    pub fn get_types(&self) -> TypeList {
        let ptr = unsafe { sys::IModule_get_types(self.ptr) };
        if ptr.is_null() {
            panic!("IModule_get_types returned NULL")
        }
        TypeList { ptr }
    }

    pub fn get_constants(&self) -> ValueList {
        let ptr = unsafe { sys::IModule_get_constants(self.ptr) };
        if ptr.is_null() {
            panic!("IModule_get_constants returned NULL")
        }
        ValueList { ptr }
    }

    pub fn get_function_count(&self) -> usize {
        unsafe { sys::IModule_get_function_count(self.ptr) }
    }

    pub fn get_function(&self, index: usize) -> String {
        unsafe {
            CStr::from_ptr(sys::IModule_get_function(self.ptr, index))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }

    pub fn functions(&self) -> FunctionIterator {
        FunctionIterator {
            module: self,
            current: 0,
            count: self.get_function_count(),
        }
    }

    pub fn get_material_count(&self) -> usize {
        unsafe { sys::IModule_get_material_count(self.ptr) }
    }

    pub fn get_material(&self, index: usize) -> String {
        unsafe {
            CStr::from_ptr(sys::IModule_get_material(self.ptr, index))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }

    pub fn materials(&self) -> MaterialIterator {
        MaterialIterator {
            module: self,
            current: 0,
            count: self.get_material_count(),
        }
    }
}

pub struct ImportIterator<'a> {
    module: &'a Module,
    current: usize,
    count: usize,
}

impl<'a> Iterator for ImportIterator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.current < self.count {
            self.current += 1;
            Some(self.module.get_import(self.current - 1))
        } else {
            None
        }
    }
}

pub struct FunctionIterator<'a> {
    module: &'a Module,
    current: usize,
    count: usize,
}

impl<'a> Iterator for FunctionIterator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.current < self.count {
            self.current += 1;
            Some(self.module.get_function(self.current - 1))
        } else {
            None
        }
    }
}

pub struct MaterialIterator<'a> {
    module: &'a Module,
    current: usize,
    count: usize,
}

impl<'a> Iterator for MaterialIterator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.current < self.count {
            self.current += 1;
            Some(self.module.get_material(self.current - 1))
        } else {
            None
        }
    }
}

impl Interface for Module {
    fn from_interface(i: sys::IInterface) -> Module {
        let i = unsafe { sys::IInterface_get_interface(i, Self::type_iid()) };
        if i.is_null() {
            panic!("Tried to convert from null interface");
        }

        Module {
            ptr: i as *mut sys::IModule_api,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IModule_type_get_iid() }
    }
}

pub enum Error {}
