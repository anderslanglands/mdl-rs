use crate::{
    base::Interface, definition::Definition, expression::ExpressionList,
    itype::TypeList, value::ValueList,
};
use mdl_sys as sys;

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
    fn from_interface(i: sys::IInterface) -> FunctionDefinition {
        let ptr = unsafe { sys::IInterface_get_interface(i, Self::type_iid()) };
        if ptr.is_null() {
            panic!("Tried to convert from null interface");
        }

        // We rlease the original pointer
        unsafe { sys::IInterface_release(i) };

        FunctionDefinition {
            ptr: ptr as *mut sys::IFunctionDefinition_api,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IFunction_definition_type_get_iid() }
    }
}

pub enum Error {}
