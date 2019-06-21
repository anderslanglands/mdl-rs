use std::os::raw::c_char;

use crate::{
    components::IMdlFactory,
    expression::{IAnnotationBlock, IAnnotationList, IExpressionList},
    itype::IType,
    scene_element::{ElementType, ISceneElement},
    transaction::ITransaction,
    type_list::ITypeList,
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DefinitionWrapper_api {
    _unused: [u8; 0],
}
pub type DefinitionWrapper = *mut DefinitionWrapper_api;

extern "C" {
    pub fn Definition_wrapper_new(
        transaction: ITransaction,
        name: *const c_char,
        factory: IMdlFactory,
    ) -> DefinitionWrapper;
    pub fn Definition_wrapper_delete(dw: DefinitionWrapper);
    pub fn Definition_wrapper_is_valid(dw: DefinitionWrapper) -> bool;
    pub fn Definition_wrapper_get_type(dw: DefinitionWrapper) -> ElementType;
    pub fn Definition_wrapper_get_mdl_definition(
        dw: DefinitionWrapper,
    ) -> *const c_char;
    pub fn Definition_wrapper_get_module(
        dw: DefinitionWrapper,
    ) -> *const c_char;
    pub fn Definition_wrapper_is_exported(dw: DefinitionWrapper) -> bool;
    pub fn Definition_wrapper_get_parameter_count(
        dw: DefinitionWrapper,
    ) -> usize;
    pub fn Definition_wrapper_get_parameter_name(
        dw: DefinitionWrapper,
        index: usize,
    ) -> *const c_char;
    pub fn Definition_wrapper_get_parameter_index(
        dw: DefinitionWrapper,
        name: *const c_char,
    ) -> isize;
    pub fn Definition_wrapper_get_parameter_types(
        dw: DefinitionWrapper,
    ) -> ITypeList;
    pub fn Definition_wrapper_get_return_type(dw: DefinitionWrapper) -> IType;
    pub fn Definition_wrapper_get_thumbnail(
        dw: DefinitionWrapper,
    ) -> *const c_char;
    pub fn Definition_wrapper_get_defaults(
        dw: DefinitionWrapper,
    ) -> IExpressionList;
    pub fn Definition_wrapper_get_default_bool(
        dw: DefinitionWrapper,
        index: usize,
        value: *mut bool,
    ) -> i32;
    pub fn Definition_wrapper_get_default_int(
        dw: DefinitionWrapper,
        index: usize,
        value: *mut i32,
    ) -> i32;
    pub fn Definition_wrapper_get_default_float(
        dw: DefinitionWrapper,
        index: usize,
        value: *mut f32,
    ) -> i32;
    pub fn Definition_wrapper_get_annotations(
        dw: DefinitionWrapper,
    ) -> IAnnotationBlock;
    pub fn Definition_wrapper_get_return_annotations(
        dw: DefinitionWrapper,
    ) -> IAnnotationBlock;
    pub fn Definition_wrapper_get_parameter_annotations(
        dw: DefinitionWrapper,
    ) -> IAnnotationList;
    pub fn Definition_wrapper_get_enable_if_conditions(
        dw: DefinitionWrapper,
    ) -> IExpressionList;
    pub fn Definition_wrapper_get_enable_if_users(
        dw: DefinitionWrapper,
        index: usize,
    ) -> usize;
    pub fn Definition_wrapper_get_enable_if_user(
        dw: DefinitionWrapper,
        index: usize,
        u_index: usize,
    ) -> usize;
    pub fn Definition_wrapper_create_instance(
        dw: DefinitionWrapper,
        args: IExpressionList,
        errors: *mut i32,
    ) -> ISceneElement;
    pub fn Definition_wrapper_get_transaction(
        dw: DefinitionWrapper,
    ) -> ITransaction;
    pub fn Definition_wrapper_get_mdl_factory(
        dw: DefinitionWrapper,
    ) -> IMdlFactory;
    pub fn Definition_wrapper_get_scene_element(
        dw: DefinitionWrapper,
    ) -> ISceneElement;
    pub fn Definition_wrapper_get_element_type(
        dw: DefinitionWrapper,
    ) -> ElementType;
    pub fn Definition_wrapper_get_name(dw: DefinitionWrapper) -> *const c_char;
}