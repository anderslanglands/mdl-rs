use crate::base::Interface;
use mdl_sys as sys;

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
    fn from_interface(i: sys::IInterface) -> CompiledMaterial {
        let ptr = unsafe { sys::IInterface_get_interface(i, Self::type_iid()) };
        if ptr.is_null() {
            panic!("Tried to convert from null interface");
        }

        // We rlease the original pointer
        unsafe { sys::IInterface_release(i) };

        CompiledMaterial {
            ptr: ptr as *mut sys::ICompiledMaterial_api,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::ICompiled_material_type_get_iid() }
    }
}

pub enum Error {}

