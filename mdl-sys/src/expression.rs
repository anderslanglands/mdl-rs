use std::os::raw::c_char;

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

    pub fn IExpression_list_release(l: IExpressionList);
    pub fn IExpression_list_retain(l: IExpressionList);
    pub fn IExpression_list_get_size(l: IExpressionList) -> usize;
    pub fn IExpression_list_get_index(l: IExpressionList, name: *const c_char) -> usize;
    pub fn IExpression_list_get_name(l: IExpressionList, index: usize) -> *const c_char;
    pub fn IExpression_list_get_expression(l: IExpressionList, index: usize) -> IExpression;
    pub fn IExpression_list_get_expression_by_name(l: IExpressionList, name: *const c_char) -> IExpression;

    pub fn IExpression_factory_release(f: IExpressionFactory);
    pub fn IExpression_factory_retain(f: IExpressionFactory);
    pub fn IExpression_factory_dump(
        f: IExpressionFactory,
        value: IExpression,
        name: *const c_char,
        depth: usize,
    ) -> *const c_char;
}

