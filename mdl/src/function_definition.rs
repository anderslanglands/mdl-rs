use crate::{
    base::Interface, definition::Definition, expression::ExpressionList,
    itype::TypeList, value::ValueList,
};
use mdl_sys as sys;

pub use sys::FunctionDefinitionSemantics;

use std::ffi::{CStr, CString};

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct FunctionDefinition {
    pub(crate) ptr: sys::IFunctionDefinition,
}

impl Drop for FunctionDefinition {
    fn drop(&mut self) {
        unsafe { sys::IFunction_definition_release(self.ptr) };
    }
}

impl Clone for FunctionDefinition {
    fn clone(&self) -> FunctionDefinition {
        unsafe {
            sys::IFunction_definition_retain(self.ptr);
        }
        FunctionDefinition { ptr: self.ptr }
    }
}

impl Definition for FunctionDefinition {
    fn get_parameter_count(&self) -> usize {
        unsafe { sys::IFunction_definition_get_parameter_count(self.ptr) }
    }

    fn get_parameter_index(&self, name: &str) -> Option<usize> {
        let name = CString::new(name).unwrap();
        let index = unsafe {
            sys::IFunction_definition_get_parameter_index(
                self.ptr,
                name.as_ptr(),
            )
        };
        if index == std::usize::MAX {
            None
        } else {
            Some(index)
        }
    }

    fn get_parameter_name(&self, index: usize) -> Option<String> {
        let ptr = unsafe {
            sys::IFunction_definition_get_parameter_name(self.ptr, index)
        };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

    fn get_parameter_types(&self) -> TypeList {
        let ptr =
            unsafe { sys::IFunction_definition_get_parameter_types(self.ptr) };
        if ptr.is_null() {
            panic!("IFunction_definition_get_parameter_types returned NULL");
        }
        TypeList { ptr }
    }

    fn get_defaults(&self) -> ExpressionList {
        let ptr = unsafe { sys::IFunction_definition_get_defaults(self.ptr) };
        if ptr.is_null() {
            panic!("IFunction_definition_get_defaults returned NULL");
        }
        ExpressionList { ptr }
    }
}

impl Interface for FunctionDefinition {
    fn from_interface_ptr(ptr: sys::IInterface) -> FunctionDefinition {
        FunctionDefinition {
            ptr: ptr as sys::IFunctionDefinition,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IFunction_definition_type_get_iid() }
    }
}

impl FunctionDefinition {
    pub fn get_mdl_name(&self) -> String {
        let ptr = unsafe { sys::IFunction_definition_get_mdl_name(self.ptr) };
        if ptr.is_null() {
            panic!("IFunction_definition_get_mdl_name returned NULL");
        }

        unsafe { CStr::from_ptr(ptr).to_str().unwrap().to_string() }
    }

    pub fn get_semantic(&self) -> FunctionDefinitionSemantics {
        unsafe { sys::IFunction_definition_get_semantic(self.ptr) }
    }
}

pub enum Error {}
