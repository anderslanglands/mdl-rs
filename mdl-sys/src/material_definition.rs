use std::os::raw::c_char;

use crate::{base::Uuid, expression::IExpressionList, type_list::ITypeList};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMaterialDefinition_api {
    _unused: [u8; 0],
}
pub type IMaterialDefinition = *mut IMaterialDefinition_api;

extern "C" {
    pub fn IMaterial_definition_release(s: IMaterialDefinition);
    pub fn IMaterial_definition_retain(s: IMaterialDefinition);
    pub fn IMaterial_definition_compare_iid(id: Uuid) -> bool;
    pub fn IMaterial_definition_type_get_iid() -> Uuid;

    pub fn IMaterial_definition_get_parameter_count(f: IMaterialDefinition) -> usize;
    pub fn IMaterial_definition_get_parameter_types(f: IMaterialDefinition) -> ITypeList;
    pub fn IMaterial_definition_get_defaults(f: IMaterialDefinition) -> IExpressionList;
    pub fn IMaterial_definition_get_parameter_name(
        f: IMaterialDefinition,
        index: usize,
    ) -> *const c_char;
    pub fn IMaterial_definition_get_parameter_index(
        f: IMaterialDefinition,
        name: *const c_char,
    ) -> usize;
}
