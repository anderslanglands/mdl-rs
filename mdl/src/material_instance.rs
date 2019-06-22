use crate::{
    base::Interface, compiled_material::CompiledMaterial,
    expression::ExpressionList, mdl_execution_context::MdlExecutionContext,
};

use mdl_sys as sys;

pub use sys::material_instance::CompilationOptions;

use std::ffi::{CStr, CString};

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct MaterialInstance {
    pub(crate) ptr: sys::IMaterialInstance,
}

impl Drop for MaterialInstance {
    fn drop(&mut self) {
        unsafe { sys::IMaterial_instance_release(self.ptr) };
    }
}

impl Clone for MaterialInstance {
    fn clone(&self) -> MaterialInstance {
        unsafe {
            sys::IMaterial_instance_retain(self.ptr);
        }
        MaterialInstance { ptr: self.ptr }
    }
}

impl MaterialInstance {
    pub fn create_compiled_material(
        &self,
        flags: CompilationOptions,
        ctx: Option<MdlExecutionContext>,
    ) -> Option<CompiledMaterial> {
        let ctx_ptr = if let Some(ctx) = ctx {
            ctx.ptr
        } else {
            std::ptr::null_mut()
        };

        let ptr = unsafe {
            sys::IMaterial_instance_create_compiled_material(
                self.ptr, flags, ctx_ptr,
            )
        };

        if ptr.is_null() {
            None
        } else {
            Some(CompiledMaterial { ptr })
        }
    }
}

impl Interface for MaterialInstance {
    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn from_interface_ptr(ptr: sys::IInterface) -> MaterialInstance {
        MaterialInstance {
            ptr: ptr as sys::IMaterialInstance,
        }
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMaterial_instance_type_get_iid() }
    }
}

pub enum Error {}

