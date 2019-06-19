use crate::components::*;
use crate::results::*;
use crate::base::Uuid;
use crate::interface::IInterface;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct INeuray_api {
    _unused: [u8; 0],
}
pub type INeuray = *mut INeuray_api;

#[link(name = "mdl-capi", kind = "static")]

extern "C" {
    pub fn load_ineuray() -> INeuray;
    pub fn ineuray_get_interface_version(n: INeuray) -> u32;
    pub fn ineuray_get_version(n: INeuray) -> *const c_char;
    pub fn ineuray_start(n: INeuray) -> INeurayStartResult;
    pub fn ineuray_shutdown(n: INeuray) -> INeurayShutdownResult;
    pub fn ineuray_get_api_component(n: INeuray, uuid: Uuid) -> IInterface;
}
