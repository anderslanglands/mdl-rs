use std::os::raw::c_char;

use crate::{expression::IExpressionList, type_list::ITypeList, base::Uuid};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IFunctionDefinition_api {
    _unused: [u8; 0],
}
pub type IFunctionDefinition = *mut IFunctionDefinition_api;

extern "C" {
    pub fn IFunction_definition_release(s: IFunctionDefinition);
    pub fn IFunction_definition_retain(s: IFunctionDefinition);
    pub fn IFunction_definition_compare_iid(id: Uuid) -> bool;
    pub fn IFunction_definition_type_get_iid() -> Uuid;

    pub fn IFunction_definition_get_parameter_count(f: IFunctionDefinition) -> usize;
    pub fn IFunction_definition_get_parameter_types(f: IFunctionDefinition) -> ITypeList;
    pub fn IFunction_definition_get_defaults(f: IFunctionDefinition) -> IExpressionList;
    pub fn IFunction_definition_get_parameter_name(
        f: IFunctionDefinition,
        index: usize,
    ) -> *const c_char;
    pub fn IFunction_definition_get_parameter_index(
        f: IFunctionDefinition,
        name: *const c_char,
    ) -> usize;
}
