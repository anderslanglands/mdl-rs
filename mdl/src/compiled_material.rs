use crate::base::Interface;
use mdl_sys as sys;

use crate::expression::{Expression, ExpressionDirectCall};

use crate::value::Value;

use std::ffi::{CStr, CString};

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct CompiledMaterial {
    pub(crate) ptr: sys::ICompiledMaterial,
}

impl Drop for CompiledMaterial {
    fn drop(&mut self) {
        unsafe { sys::ICompiled_material_release(self.ptr) };
    }
}

impl Clone for CompiledMaterial {
    fn clone(&self) -> CompiledMaterial {
        unsafe {
            sys::ICompiled_material_retain(self.ptr);
        }
        CompiledMaterial { ptr: self.ptr }
    }
}

impl Interface for CompiledMaterial {
    fn from_interface_ptr(ptr: sys::IInterface) -> CompiledMaterial {
        CompiledMaterial {
            ptr: ptr as sys::ICompiledMaterial,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::ICompiled_material_type_get_iid() }
    }
}

impl CompiledMaterial {
    pub fn get_body(&self) -> ExpressionDirectCall {
        let ptr = unsafe { sys::ICompiled_material_get_body(self.ptr) };
        if ptr.is_null() {
            panic!("ICompiled_material_get_body returned NULL");
        }

        ExpressionDirectCall { ptr }
    }

    pub fn get_parameter_count(&self) -> usize {
        unsafe { sys::ICompiled_material_get_parameter_count(self.ptr) }
    }

    pub fn get_parameter_name(&self, index: usize) -> Option<String> {
        let ptr = unsafe {
            sys::ICompiled_material_get_parameter_name(self.ptr, index)
        };
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(CStr::from_ptr(ptr).to_str().unwrap().to_string()) }
        }
    }

    pub fn get_argument(&self, index: usize) -> Option<Value> {
        let ptr =
            unsafe { sys::ICompiled_material_get_argument(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(Value { ptr })
        }
    }

    pub fn get_temporary_count(&self) -> usize {
        unsafe { sys::ICompiled_material_get_temporary_count(self.ptr) }
    }

    pub fn get_temporary(&self, index: usize) -> Option<Expression> {
        let ptr =
            unsafe { sys::ICompiled_material_get_temporary(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(Expression { ptr })
        }
    }
}

pub enum Error {}

