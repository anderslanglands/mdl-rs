use std::os::raw::c_char;

use crate::base::Uuid;

#[repr(u32)]
pub enum ValueKind {
    Bool,
    Int,
    Enum,
    Float,
    Double,
    String,
    Vector,
    Matrix,
    Color,
    Array,
    Struct,
    InvalidDf,
    Texture,
    LightProfile,
    BsdfMeasurement,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IValue_api {
    _unused: [u8; 0],
}
pub type IValue = *mut IValue_api;

extern "C" {
    pub fn IValue_release(i: IValue);
    pub fn IValue_retain(i: IValue);
    pub fn IValue_compare_iid(id: Uuid) -> bool;
    pub fn IValue_type_get_iid() -> Uuid;
    pub fn IValue_get_kind(i: IValue) -> ValueKind;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IValueList_api {
    _unused: [u8; 0],
}
pub type IValueList = *mut IValueList_api;

extern "C" {
    pub fn IValue_list_release(l: IValueList);
    pub fn IValue_list_retain(l: IValueList);
    pub fn IValue_list_compare_iid(id: Uuid) -> bool;
    pub fn IValue_list_type_get_iid() -> Uuid;
    pub fn IValue_list_get_size(l: IValueList) -> usize;
    pub fn IValue_list_get_index(l: IValueList, name: *const c_char) -> usize;
    pub fn IValue_list_get_name(l: IValueList, index: usize) -> *const c_char;
    pub fn IValue_list_get_value(l: IValueList, index: usize) -> IValue;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IValueCompound_api {
    _unused: [u8; 0],
}
pub type IValueCompound = *mut IValueCompound_api;

extern "C" {
    pub fn IValue_compound_release(l: IValueCompound);
    pub fn IValue_compound_retain(l: IValueCompound);
    pub fn IValue_compound_compare_iid(id: Uuid) -> bool;
    pub fn IValue_compound_type_get_iid() -> Uuid;
    pub fn IValue_compound_get_size(l: IValueCompound) -> usize;
    pub fn IValue_compound_get_value(l: IValueCompound, index: usize)
        -> IValue;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IValueFactory_api {
    _unused: [u8; 0],
}
pub type IValueFactory = *mut IValueFactory_api;

extern "C" {
    pub fn IValue_factory_release(f: IValueFactory);
    pub fn IValue_factory_retain(f: IValueFactory);
    pub fn IValue_factory_dump(
        f: IValueFactory,
        value: IValue,
        name: *const c_char,
        depth: usize,
    ) -> *const c_char;
}
