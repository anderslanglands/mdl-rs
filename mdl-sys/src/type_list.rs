use crate::IType;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ITypeList_api {
    _unused: [u8; 0],
}
pub type ITypeList = *mut ITypeList_api;

extern "C" {
    pub fn IType_list_release(s: ITypeList);
    pub fn IType_list_retain(s: ITypeList);
    pub fn IType_list_get_size(l: ITypeList) -> usize;
    pub fn IType_list_get_index(l: ITypeList, name: *const c_char) -> usize;
    pub fn IType_list_get_name(l: ITypeList, index: usize) -> *const c_char;
    pub fn IType_list_get_type(l: ITypeList, index: usize) -> IType;
}
