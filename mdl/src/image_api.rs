use mdl_sys as sys;

use crate::{base::Interface, neuray::ApiComponent};

pub struct ImageApi {
    pub(crate) ptr: sys::IImageApi,
}

impl Drop for ImageApi {
    fn drop(&mut self) {
        unsafe { sys::IImage_api_release(self.ptr) };
    }
}

impl Clone for ImageApi {
    fn clone(&self) -> ImageApi {
        unsafe { sys::IImage_api_retain(self.ptr) };
        ImageApi { ptr: self.ptr }
    }
}

impl ImageApi {}

impl Interface for ImageApi {
    fn from_interface_ptr(ptr: sys::IInterface) -> ImageApi {
        ImageApi {
            ptr: ptr as sys::IImageApi,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IImage_api_type_get_iid() }
    }
}

impl ApiComponent for ImageApi {}
