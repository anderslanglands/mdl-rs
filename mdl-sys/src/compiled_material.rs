use std::os::raw::c_char;

use crate::base::Uuid;

use crate::expression::{IExpression, IExpressionDirectCall};

use crate::value::IValue;

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
    pub fn ICompiled_material_get_body(
        n: ICompiledMaterial,
    ) -> IExpressionDirectCall;
    pub fn ICompiled_material_get_parameter_count(
        m: ICompiledMaterial,
    ) -> usize;
    pub fn ICompiled_material_get_parameter_name(
        m: ICompiledMaterial,
        index: usize,
    ) -> *const c_char;
    pub fn ICompiled_material_get_argument(
        m: ICompiledMaterial,
        index: usize,
    ) -> IValue;
    pub fn ICompiled_material_get_temporary_count(
        m: ICompiledMaterial,
    ) -> usize;
    pub fn ICompiled_material_get_temporary(
        m: ICompiledMaterial,
        index: usize,
    ) -> IExpression;
}

