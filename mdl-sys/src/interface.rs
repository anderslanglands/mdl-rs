use crate::base::Uuid;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IInterface_api {
    _unused: [u8; 0],
}
pub type IInterface = *mut IInterface_api;

extern "C" {
    pub fn IInterface_get_iid(i: IInterface);
    pub fn IInterface_get_interface(i: IInterface, id: Uuid) -> IInterface;
}
