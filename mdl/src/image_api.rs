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
    fn from_interface(i: sys::IInterface) -> ImageApi {
        let i = unsafe { sys::IInterface_get_interface(i, Self::type_iid()) };
        if i.is_null() {
            panic!("Tried to convert from null interface");
        }

        ImageApi {
            ptr: i as *mut sys::IImageApi_api,
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
