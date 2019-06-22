use std::os::raw::c_char;

use crate::base::Uuid;

use crate::itype::IType;

use crate::value::IValue;

#[repr(u32)]
pub enum ExpressionKind {
    Constant,
    Call,
    Parameter,
    DirectCall,
    Temporary,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IExpression_api {
    _unused: [u8; 0],
}
pub type IExpression = *mut IExpression_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IExpressionList_api {
    _unused: [u8; 0],
}
pub type IExpressionList = *mut IExpressionList_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IExpressionFactory_api {
    _unused: [u8; 0],
}
pub type IExpressionFactory = *mut IExpressionFactory_api;

extern "C" {
    pub fn IExpression_release(i: IExpression);
    pub fn IExpression_retain(i: IExpression);
    pub fn IExpression_compare_iid(id: Uuid) -> bool;
    pub fn IExpression_type_get_iid() -> Uuid;
    pub fn IExpression_get_kind(i: IExpression) -> ExpressionKind;
    pub fn IExpression_get_type(i: IExpression) -> IType;

    pub fn IExpression_list_release(l: IExpressionList);
    pub fn IExpression_list_retain(l: IExpressionList);
    pub fn IExpression_list_get_size(l: IExpressionList) -> usize;
    pub fn IExpression_list_get_index(
        l: IExpressionList,
        name: *const c_char,
    ) -> usize;
    pub fn IExpression_list_get_name(
        l: IExpressionList,
        index: usize,
    ) -> *const c_char;
    pub fn IExpression_list_get_expression(
        l: IExpressionList,
        index: usize,
    ) -> IExpression;
    pub fn IExpression_list_get_expression_by_name(
        l: IExpressionList,
        name: *const c_char,
    ) -> IExpression;

    pub fn IExpression_factory_release(f: IExpressionFactory);
    pub fn IExpression_factory_retain(f: IExpressionFactory);
    pub fn IExpression_factory_dump(
        f: IExpressionFactory,
        value: IExpression,
        name: *const c_char,
        depth: usize,
    ) -> *const c_char;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IExpressionDirectCall_api {
    _unused: [u8; 0],
}
pub type IExpressionDirectCall = *mut IExpressionDirectCall_api;

extern "C" {
    pub fn IExpression_direct_call_release(i: IExpressionDirectCall);
    pub fn IExpression_direct_call_retain(i: IExpressionDirectCall);
    pub fn IExpression_direct_call_compare_iid(id: Uuid) -> bool;
    pub fn IExpression_direct_call_type_get_iid() -> Uuid;
    pub fn IExpression_direct_call_get_definition(
        i: IExpressionDirectCall,
    ) -> *const c_char;
    pub fn IExpression_direct_call_get_arguments(
        i: IExpressionDirectCall,
    ) -> IExpressionList;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IExpressionCall_api {
    _unused: [u8; 0],
}
pub type IExpressionCall = *mut IExpressionCall_api;

extern "C" {
    pub fn IExpression_call_release(i: IExpressionCall);
    pub fn IExpression_call_retain(i: IExpressionCall);
    pub fn IExpression_call_compare_iid(id: Uuid) -> bool;
    pub fn IExpression_call_type_get_iid() -> Uuid;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IExpressionParameter_api {
    _unused: [u8; 0],
}
pub type IExpressionParameter = *mut IExpressionParameter_api;

extern "C" {
    pub fn IExpression_parameter_release(i: IExpressionParameter);
    pub fn IExpression_parameter_retain(i: IExpressionParameter);
    pub fn IExpression_parameter_compare_iid(id: Uuid) -> bool;
    pub fn IExpression_parameter_type_get_iid() -> Uuid;
    pub fn IExpression_parameter_get_index(i: IExpressionParameter) -> usize;
    pub fn IExpression_parameter_set_index(
        i: IExpressionParameter,
        index: usize,
    );
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IExpressionConstant_api {
    _unused: [u8; 0],
}
pub type IExpressionConstant = *mut IExpressionConstant_api;

extern "C" {
    pub fn IExpression_constant_release(i: IExpressionConstant);
    pub fn IExpression_constant_retain(i: IExpressionConstant);
    pub fn IExpression_constant_compare_iid(id: Uuid) -> bool;
    pub fn IExpression_constant_type_get_iid() -> Uuid;
    pub fn IExpression_constant_get_value(i: IExpressionConstant) -> IValue;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IExpressionTemporary_api {
    _unused: [u8; 0],
}
pub type IExpressionTemporary = *mut IExpressionTemporary_api;

extern "C" {
    pub fn IExpression_temporary_release(i: IExpressionTemporary);
    pub fn IExpression_temporary_retain(i: IExpressionTemporary);
    pub fn IExpression_temporary_compare_iid(id: Uuid) -> bool;
    pub fn IExpression_temporary_type_get_iid() -> Uuid;
    pub fn IExpression_temporary_get_index(i: IExpressionTemporary) -> usize;
    pub fn IExpression_temporary_set_index(
        i: IExpressionTemporary,
        index: usize,
    );
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IAnnotationBlock_api {
    _unused: [u8; 0],
}
pub type IAnnotationBlock = *mut IAnnotationBlock_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IAnnotationList_api {
    _unused: [u8; 0],
}
pub type IAnnotationList = *mut IAnnotationList_api;
