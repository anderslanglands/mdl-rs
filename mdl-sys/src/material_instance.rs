use crate::{
    base::Uuid, compiled_material::ICompiledMaterial,
    expression::IExpressionList, mdl_execution_context::IMdlExecutionContext,
};

use bitflags::bitflags;

use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMaterialInstance_api {
    _unused: [u8; 0],
}
pub type IMaterialInstance = *mut IMaterialInstance_api;

bitflags! {
    #[repr(C)]
    pub struct CompilationOptions: u32 {
        const DEFAULT_OPTIONS = 0;
        const CLASS_COMPILATION = 1;
    }
}

extern "C" {
    pub fn IMaterial_instance_release(s: IMaterialInstance);
    pub fn IMaterial_instance_retain(s: IMaterialInstance);
    pub fn IMaterial_instance_compare_iid(id: Uuid) -> bool;
    pub fn IMaterial_instance_type_get_iid() -> Uuid;
    pub fn IMaterial_instance_create_compiled_material(
        i: IMaterialInstance,
        flags: CompilationOptions,
        ctx: IMdlExecutionContext,
    ) -> ICompiledMaterial;
}

