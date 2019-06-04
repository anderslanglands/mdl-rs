use crate::base::Uuid;
use crate::components::IVersion;
use std::os::raw::c_char;

extern "C" {
    pub fn IVersion_release(i: IVersion);
    pub fn IVersion_get_product_name(i: IVersion) -> *const c_char;
    pub fn IVersion_get_product_version(i: IVersion) -> *const c_char;
    pub fn IVersion_get_build_number(i: IVersion) -> *const c_char;
    pub fn IVersion_get_build_date(i: IVersion) -> *const c_char;
    pub fn IVersion_get_build_platform(i: IVersion) -> *const c_char;
    pub fn IVersion_get_string(i: IVersion) -> *const c_char;
    pub fn IVersion_get_neuray_iid(i: IVersion) -> Uuid;
}
