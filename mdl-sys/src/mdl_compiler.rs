use crate::{
    components::IMdlCompiler,
    mdl_execution_context::IMdlExecutionContext,
    results::{AddPathResult, BackendSetOptionResult, BooleanResult, LoadModuleResult},
    transaction::ITransaction,
    base::Uuid,
};

use std::os::raw::c_char;

#[repr(i32)]
pub enum MdlBackendKind {
    CudaPtx,
    LlvmIr,
    Glsl,
    Native,
}

extern "C" {
    pub fn IMdl_compiler_release(c: IMdlCompiler);
    pub fn IMdl_compiler_retain(c: IMdlCompiler);
    pub fn IMdl_compiler_type_get_iid() -> Uuid;
    pub fn IMdl_compiler_add_module_path(c: IMdlCompiler, path: *const c_char) -> AddPathResult;
    pub fn IMdl_compiler_remove_module_path(c: IMdlCompiler, path: *const c_char) -> AddPathResult;
    pub fn IMdl_compiler_clear_module_paths(c: IMdlCompiler);
    pub fn IMdl_compiler_load_plugin_library(c: IMdlCompiler, path: *const c_char)
        -> BooleanResult;
    pub fn IMdl_compiler_get_backend(c: IMdlCompiler, kind: MdlBackendKind) -> IMdlBackend;
    pub fn IMdl_compiler_load_module(
        c: IMdlCompiler,
        transaction: ITransaction,
        name: *const c_char,
        ctx: IMdlExecutionContext,
    ) -> LoadModuleResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlBackend_api {
    _unused: [u8; 0],
}
pub type IMdlBackend = *mut IMdlBackend_api;

extern "C" {
    pub fn IMdl_backend_release(be: IMdlBackend);
    pub fn IMdl_backend_set_option(
        be: IMdlBackend,
        name: *const c_char,
        value: *const c_char,
    ) -> BackendSetOptionResult;
}
