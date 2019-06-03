use crate::components::IMdlFactory;
use crate::mdl_execution_context::IMdlExecutionContext;
use std::os::raw::c_char;

extern "C" {
    pub fn IMdl_factory_create_execution_context(f: IMdlFactory) -> IMdlExecutionContext;
}
