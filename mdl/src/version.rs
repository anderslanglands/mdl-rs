use mdl_sys as sys;
use std::ffi::CStr;


use crate::base::Interface;
use crate::neuray::ApiComponent;
pub struct Version {
    pub(crate) ptr: sys::IVersion,
}

impl Drop for Version {
    fn drop(&mut self) {
        unsafe { sys::IVersion_release(self.ptr) };
    }
}

impl Version {
    pub fn get_product_name(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::IVersion_get_product_name(self.ptr))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }
    pub fn get_product_version(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::IVersion_get_product_version(self.ptr))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }
    pub fn get_build_number(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::IVersion_get_build_number(self.ptr))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }
    pub fn get_build_date(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::IVersion_get_build_date(self.ptr))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }
    pub fn get_build_platform(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::IVersion_get_build_platform(self.ptr))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }
    pub fn get_string(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::IVersion_get_string(self.ptr))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }
    pub fn get_neuray_iid(&self) -> sys::Uuid {
        unsafe { sys::IVersion_get_neuray_iid(self.ptr) }
    }
}

impl Interface for Version {
    fn from_interface_ptr(ptr: sys::IInterface) -> Version {
        Version {
            ptr: ptr as sys::IVersion,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IVersion_type_get_iid() }
    }
}

impl ApiComponent for Version {}
