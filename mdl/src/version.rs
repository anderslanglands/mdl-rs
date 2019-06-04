use mdl_sys as sys;
use std::ffi::CStr;

pub struct Version {
    pub(crate) v: sys::IVersion,
}

impl Version {
    pub fn get_product_name(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::IVersion_get_product_name(self.v))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }
    pub fn get_product_version(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::IVersion_get_product_version(self.v))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }
    pub fn get_build_number(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::IVersion_get_build_number(self.v))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }
    pub fn get_build_date(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::IVersion_get_build_date(self.v))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }
    pub fn get_build_platform(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::IVersion_get_build_platform(self.v))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }
    pub fn get_string(&self) -> String {
        unsafe {
            CStr::from_ptr(sys::IVersion_get_string(self.v))
                .to_string_lossy()
                .to_owned()
                .to_string()
        }
    }
    pub fn get_neuray_iid(&self) -> sys::Uuid {
        unsafe { sys::IVersion_get_neuray_iid(self.v) }
    }
}
