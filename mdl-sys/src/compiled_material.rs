use std::os::raw::c_char;

use crate::base::Uuid;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ICompiledMaterial_api {
    _unused: [u8; 0],
}
pub type ICompiledMaterial = *mut ICompiledMaterial_api;

extern "C" {
    pub fn ICompiled_material_release(s: ICompiledMaterial);
    pub fn ICompiled_material_retain(s: ICompiledMaterial);
    pub fn ICompiled_material_compare_iid(id: Uuid) -> bool;
    pub fn ICompiled_material_type_get_iid() -> Uuid;
}

