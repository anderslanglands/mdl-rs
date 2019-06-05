use crate::base::Uuid;
use crate::value::IValueList;
use crate::ITypeList;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IModule_api {
    _unused: [u8; 0],
}
pub type IModule = *mut IModule_api;

extern "C" {
    pub fn IModule_release(s: IModule);
    pub fn IModule_retain(s: IModule);
    pub fn IModule_compare_iid(id: Uuid) -> bool;
    pub fn IModule_type_get_iid() -> Uuid;
    pub fn IModule_get_filename(m: IModule) -> *const c_char;
    pub fn IModule_get_mdl_name(m: IModule) -> *const c_char;
    pub fn IModule_get_import_count(m: IModule) -> usize;
    pub fn IModule_get_import(m: IModule, index: usize) -> *const c_char;
    pub fn IModule_get_types(m: IModule) -> ITypeList;
    pub fn IModule_get_constants(m: IModule) -> IValueList;
    pub fn IModule_get_function(m: IModule, index: usize) -> *const c_char;
    pub fn IModule_get_function_count(m: IModule) -> usize;
    pub fn IModule_get_material(m: IModule, index: usize) -> *const c_char;
    pub fn IModule_get_material_count(m: IModule) -> usize;
}
