use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IValue_api {
    _unused: [u8; 0],
}
pub type IValue = *mut IValue_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IValueList_api {
    _unused: [u8; 0],
}
pub type IValueList = *mut IValueList_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IValueFactory_api {
    _unused: [u8; 0],
}
pub type IValueFactory = *mut IValueFactory_api;

extern "C" {
    pub fn IValue_release(i: IValue);
    pub fn IValue_retain(i: IValue);

    pub fn IValue_list_release(l: IValueList);
    pub fn IValue_list_retain(l: IValueList);
    pub fn IValue_list_get_size(l: IValueList) -> usize;
    pub fn IValue_list_get_index(l: IValueList, name: *const c_char) -> usize;
    pub fn IValue_list_get_name(l: IValueList, index: usize) -> *const c_char;
    pub fn IValue_list_get_value(l: IValueList, index: usize) -> IValue;

    pub fn IValue_factory_release(f: IValueFactory);
    pub fn IValue_factory_retain(f: IValueFactory);
    pub fn IValue_factory_dump(
        f: IValueFactory,
        value: IValue,
        name: *const c_char,
        depth: usize,
    ) -> *const c_char;
}
