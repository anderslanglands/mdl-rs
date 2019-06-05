use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IType_api {
    _unused: [u8; 0],
}
pub type IType = *mut IType_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ITypeFactory_api {
    _unused: [u8; 0],
}
pub type ITypeFactory = *mut ITypeFactory_api;

extern "C" {
    pub fn IType_release(s: IType);
    pub fn IType_retain(s: IType);
    pub fn IType_factory_release(f: ITypeFactory);
    pub fn IType_factory_retain(f: ITypeFactory);
    pub fn IType_factory_dump(f: ITypeFactory, typ: IType, depth: usize) -> *const c_char;
}
