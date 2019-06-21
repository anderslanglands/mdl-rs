use crate::base::Uuid;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IImageApi_api {
    _unused: [u8; 0],
}
pub type IImageApi = *mut IImageApi_api;

extern "C" {
    pub fn IImage_api_release(i: IImageApi);
    pub fn IImage_api_retain(i: IImageApi);
    pub fn IImage_api_type_get_iid() -> Uuid;
}
