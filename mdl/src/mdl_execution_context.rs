use mdl_sys as sys;
use std::ffi::CString;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct MdlExecutionContext {
    pub(crate) ptr: sys::IMdlExecutionContext,
}

impl Drop for MdlExecutionContext {
    fn drop(&mut self) {
        unsafe { sys::IMdl_execution_context_release(self.ptr) };
    }
}

impl Clone for MdlExecutionContext {
    fn clone(&self) -> MdlExecutionContext {
        unsafe {
            sys::IMdl_execution_context_retain(self.ptr);
        }
        MdlExecutionContext { ptr: self.ptr }
    }
}

impl MdlExecutionContext {

}

pub enum Error {}


