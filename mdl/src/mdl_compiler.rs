use mdl_sys as sys;

pub use sys::mdl_compiler::MdlBackendKind;

use crate::{
    base::Interface, compiled_material::CompiledMaterial,
    mdl_execution_context::MdlExecutionContext, neuray::ApiComponent,
    transaction::Transaction,
};
use std::ffi::{CStr, CString};
use std::path::Path;

use err_derive::Error;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct MdlCompiler {
    pub(crate) ptr: sys::IMdlCompiler,
}

impl Drop for MdlCompiler {
    fn drop(&mut self) {
        unsafe { sys::IMdl_compiler_release(self.ptr) };
    }
}

impl Clone for MdlCompiler {
    fn clone(&self) -> MdlCompiler {
        unsafe { sys::IMdl_compiler_retain(self.ptr) };
        MdlCompiler { ptr: self.ptr }
    }
}

impl MdlCompiler {
    pub fn add_module_path<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref().to_str().unwrap();
        let cpath = CString::new(path).unwrap();
        let result = unsafe {
            sys::IMdl_compiler_add_module_path(self.ptr, cpath.as_ptr())
        };
        match result {
            sys::AddPathResult::Success => Ok(()),
            sys::AddPathResult::InvalidParameters => {
                Err(Error::InvalidParameters {
                    param: path.to_string(),
                })
            }
            sys::AddPathResult::InvalidPath => Err(Error::InvalidPath {
                path: path.to_string(),
            }),
        }
    }

    pub fn load_plugin_library(&mut self, name: &str) -> Result<()> {
        let cname = CString::new(name).unwrap();
        let result = unsafe {
            sys::IMdl_compiler_load_plugin_library(self.ptr, cname.as_ptr())
        };
        if result != sys::BooleanResult::Success {
            return Err(Error::LoadPluginFailed {
                plugin: name.to_string(),
            });
        }

        Ok(())
    }

    pub fn load_module(
        &self,
        transaction: &Transaction,
        name: &str,
        ctx: &MdlExecutionContext,
    ) -> Result<()> {
        let cname = CString::new(name).unwrap();
        let result = unsafe {
            sys::IMdl_compiler_load_module(
                self.ptr,
                transaction.ptr,
                cname.as_ptr(),
                ctx.ptr,
            )
        };
        match result {
            sys::results::LoadModuleResult::Success => Ok(()),
            sys::results::LoadModuleResult::ModuleNameInvalid => {
                Err(Error::ModuleNameInvalid {
                    name: name.to_string(),
                })
            }
            sys::results::LoadModuleResult::FailedToLoadModule => {
                Err(Error::FailedToLoadModule {
                    name: name.to_string(),
                })
            }
            sys::results::LoadModuleResult::NameAlreadyInUse => {
                Err(Error::NameAlreadyInUse {
                    name: name.to_string(),
                })
            }
            sys::results::LoadModuleResult::InitializationFailed => {
                Err(Error::InitializationFailed {
                    name: name.to_string(),
                })
            }
        }
    }

    pub fn get_backend(&self, kind: MdlBackendKind) -> Option<MdlBackend> {
        let ptr = unsafe { sys::IMdl_compiler_get_backend(self.ptr, kind) };
        if ptr.is_null() {
            None
        } else {
            Some(MdlBackend { ptr })
        }
    }
}

impl Interface for MdlCompiler {
    fn from_interface(i: sys::IInterface) -> MdlCompiler {
        let i = unsafe { sys::IInterface_get_interface(i, Self::type_iid()) };
        if i.is_null() {
            panic!("Tried to convert from null interface");
        }

        MdlCompiler {
            ptr: i as *mut sys::IMdlCompiler_api,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMdl_compiler_type_get_iid() }
    }
}

impl ApiComponent for MdlCompiler {}

pub struct MdlBackend {
    ptr: sys::IMdlBackend,
}

impl MdlBackend {
    pub fn set_option(&self, name: &str, value: &str) -> Result<()> {
        let cname = CString::new(name).unwrap();
        let cvalue = CString::new(value).unwrap();
        let result = unsafe {
            sys::IMdl_backend_set_option(
                self.ptr,
                cname.as_ptr(),
                cvalue.as_ptr(),
            )
        };
        match result {
            sys::BackendSetOptionResult::Success => Ok(()),
            sys::BackendSetOptionResult::UnknownOption => {
                Err(Error::UnknownOption(name.to_string()))
            }
            sys::BackendSetOptionResult::UnsupportedValue => Err(
                Error::UnsupportedValue(value.to_string(), name.to_string()),
            ),
        }
    }

    pub fn translate_material_expression(
        &self,
        transaction: &Transaction,
        compiled_material: &CompiledMaterial,
        path: &str,
        fname: &str,
        ctx: &MdlExecutionContext,
    ) -> Option<TargetCode> {
        let cpath = CString::new(path).unwrap();
        let cfname = CString::new(fname).unwrap();
        let ptr = unsafe {
            sys::IMdl_backend_translate_material_expression(
                self.ptr,
                transaction.ptr,
                compiled_material.ptr,
                cpath.as_ptr(),
                cfname.as_ptr(),
                ctx.ptr,
            )
        };
        if ptr.is_null() {
            None
        } else {
            Some(TargetCode { ptr })
        }
    }
}

impl Interface for MdlBackend {
    fn from_interface(i: sys::IInterface) -> MdlBackend {
        let i = unsafe { sys::IInterface_get_interface(i, Self::type_iid()) };
        if i.is_null() {
            panic!("Tried to convert from null interface");
        }

        MdlBackend {
            ptr: i as *mut sys::IMdlBackend_api,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMdl_backend_type_get_iid() }
    }
}

pub struct TargetCode {
    pub(crate) ptr: sys::ITargetCode,
}

impl Drop for TargetCode {
    fn drop(&mut self) {
        unsafe { sys::ITarget_code_release(self.ptr) };
    }
}

impl Clone for TargetCode {
    fn clone(&self) -> TargetCode {
        unsafe {
            sys::ITarget_code_retain(self.ptr);
        }
        TargetCode { ptr: self.ptr }
    }
}

impl TargetCode {
    pub fn get_code(&self) -> Option<String> {
        let ptr = unsafe { sys::ITarget_code_get_code(self.ptr) };
        let sz = unsafe { sys::ITarget_code_get_code_size(self.ptr) };
        if ptr.is_null() || sz == 0 {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }
}

impl Interface for TargetCode {
    fn from_interface(i: sys::IInterface) -> TargetCode {
        let ptr = unsafe { sys::IInterface_get_interface(i, Self::type_iid()) };
        if ptr.is_null() {
            panic!("Tried to convert from null interface");
        }

        // We rlease the original pointer
        unsafe { sys::IInterface_release(i) };

        TargetCode {
            ptr: ptr as *mut sys::ITargetCode_api,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::ITarget_code_type_get_iid() }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "Invalid path: '{}'", path)]
    InvalidPath { path: String },
    #[error(display = "Invalid parameters: '{}'", param)]
    InvalidParameters { param: String },
    #[error(display = "Failed to load plugin: '{}'", plugin)]
    LoadPluginFailed { plugin: String },
    #[error(display = "Tried to load invalid module name '{}'", name)]
    ModuleNameInvalid { name: String },
    #[error(display = "Failed to load or compile module '{}'", name)]
    FailedToLoadModule { name: String },
    #[error(
        display = "DB name for requested module '{}' is already in use",
        name
    )]
    NameAlreadyInUse { name: String },
    #[error(display = "Failed to initialize loaded module '{}'", name)]
    InitializationFailed { name: String },
    #[error(display = "Unknown option '{}'", _0)]
    UnknownOption(String),
    #[error(display = "Unsupported value '{}' for option '{}'", _0, _1)]
    UnsupportedValue(String, String),
}
