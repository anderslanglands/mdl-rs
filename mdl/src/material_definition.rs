use crate::{
    base::Interface, definition::Definition, expression::ExpressionList, itype::TypeList,
    value::ValueList,
};
use mdl_sys as sys;

use std::ffi::{CStr, CString};

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct MaterialDefinition {
    pub(crate) ptr: sys::IMaterialDefinition,
}

impl Drop for MaterialDefinition {
    fn drop(&mut self) {
        unsafe { sys::IMaterial_definition_release(self.ptr) };
    }
}

impl Clone for MaterialDefinition {
    fn clone(&self) -> MaterialDefinition {
        unsafe {
            sys::IMaterial_definition_retain(self.ptr);
        }
        MaterialDefinition { ptr: self.ptr }
    }
}

impl Definition for MaterialDefinition {
    fn get_parameter_count(&self) -> usize {
        unsafe { sys::IMaterial_definition_get_parameter_count(self.ptr) }
    }

    fn get_parameter_index(&self, name: &str) -> Option<usize> {
        let name = CString::new(name).unwrap();
        let index =
            unsafe { sys::IMaterial_definition_get_parameter_index(self.ptr, name.as_ptr()) };
        if index == std::usize::MAX {
            None
        } else {
            Some(index)
        }
    }

    fn get_parameter_name(&self, index: usize) -> Option<String> {
        let ptr = unsafe { sys::IMaterial_definition_get_parameter_name(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string() })
        }
    }

    fn get_parameter_types(&self) -> TypeList {
        let ptr = unsafe { sys::IMaterial_definition_get_parameter_types(self.ptr) };
        if ptr.is_null() {
            panic!("IMaterial_definition_get_parameter_types returned NULL");
        }
        TypeList { ptr }
    }

    fn get_defaults(&self) -> ExpressionList {
        let ptr = unsafe { sys::IMaterial_definition_get_defaults(self.ptr) };
        if ptr.is_null() {
            panic!("IMaterial_definition_get_defaults returned NULL");
        }
        ExpressionList { ptr }
    }
}

impl Interface for MaterialDefinition {
    fn from_interface(i: sys::IInterface) -> MaterialDefinition {
        let ptr = unsafe { sys::IInterface_get_interface(i, Self::type_iid()) };
        if ptr.is_null() {
            panic!("Tried to convert from null interface");
        }

        // We rlease the original pointer
        unsafe { sys::IInterface_release(i) };

        MaterialDefinition {
            ptr: ptr as *mut sys::IMaterialDefinition_api,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMaterial_definition_type_get_iid() }
    }
}

pub enum Error {}
