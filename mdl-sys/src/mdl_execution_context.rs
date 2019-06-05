use crate::results::SetOptionResult;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlExecutionContext_api {
    _unused: [u8; 0],
}
pub type IMdlExecutionContext = *mut IMdlExecutionContext_api;

extern "C" {
    pub fn IMdl_execution_context_release(c: IMdlExecutionContext);
    pub fn IMdl_execution_context_retain(c: IMdlExecutionContext);
    pub fn IMdl_execution_context_set_option_string(
        c: IMdlExecutionContext,
        name: *const c_char,
        value: *const c_char,
    ) -> SetOptionResult;

    pub fn IMdl_execution_context_set_option_float(
        c: IMdlExecutionContext,
        name: *const c_char,
        value: f32,
    ) -> SetOptionResult;

    pub fn IMdl_execution_context_set_option_bool(
        c: IMdlExecutionContext,
        name: *const c_char,
        value: bool,
    ) -> SetOptionResult;
}
