use crate::components::IMdlCompiler;
use crate::results::{AddPathResult, BackendSetOptionResult, BooleanResult};
use std::os::raw::c_char;

#[repr(i32)]
pub enum MdlBackendKind {
    CudaPtx,
    LlvmIr,
    Glsl,
    Native,
}

extern "C" {
    pub fn IMdl_compiler_add_module_path(c: IMdlCompiler, path: *const c_char) -> AddPathResult;
    pub fn IMdl_compiler_remove_module_path(c: IMdlCompiler, path: *const c_char) -> AddPathResult;
    pub fn IMdl_compiler_clear_module_paths(c: IMdlCompiler);
    pub fn IMdl_compiler_load_plugin_library(c: IMdlCompiler, path: *const c_char)
        -> BooleanResult;
    pub fn IMdl_compiler_get_backend(c: IMdlCompiler, kind: MdlBackendKind) -> IMdlBackend;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlBackend_api {
    _unused: [u8; 0],
}
pub type IMdlBackend = *mut IMdlBackend_api;

extern "C" {
    pub fn IMdl_backend_set_option(
        be: IMdlBackend,
        name: *const c_char,
        value: *const c_char,
    ) -> BackendSetOptionResult;
}