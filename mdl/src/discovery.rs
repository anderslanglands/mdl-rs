use mdl_sys as sys;

use std::ffi::CStr;

use crate::base::Interface;
use crate::neuray::ApiComponent;
pub use sys::MdlInfoKind;

pub struct MdlInfoBase {
    pub(crate) ptr: sys::IMdlInfo,
}

impl MdlInfo for MdlInfoBase {
    fn as_mdl_info(&self) -> sys::IMdlInfo {
        self.ptr as sys::IMdlInfo
    }
}

impl Interface for MdlInfoBase {
    fn from_interface_ptr(ptr: sys::IInterface) -> MdlInfoBase {
        MdlInfoBase {
            ptr: ptr as sys::IMdlInfo,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMdl_info_type_get_iid() }
    }
}

pub trait MdlInfo: Interface {
    fn as_mdl_info(&self) -> sys::IMdlInfo;

    fn get_kind(&self) -> MdlInfoKind {
        unsafe { sys::IMdl_info_get_kind(self.as_mdl_info()) }
    }

    fn get_qualified_name(&self) -> Option<String> {
        let ptr =
            unsafe { sys::IMdl_info_get_qualified_name(self.as_mdl_info()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

    fn get_simple_name(&self) -> Option<String> {
        let ptr = unsafe { sys::IMdl_info_get_simple_name(self.as_mdl_info()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

}

pub struct MdlModuleInfo {
    pub(crate) ptr: sys::IMdlModuleInfo,
}

impl MdlModuleInfo {
    pub fn get_search_path_index(&self) -> usize {
        unsafe { sys::IMdl_module_info_get_search_path_index(self.ptr) }
    }

    pub fn get_search_path(&self) -> Option<String> {
        let ptr = unsafe { sys::IMdl_module_info_get_search_path(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }
    pub fn get_resolved_path(&self) -> Option<String> {
        let ptr = unsafe { sys::IMdl_module_info_get_resolved_path(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

    pub fn in_archive(&self) -> bool {
        unsafe { sys::IMdl_module_info_in_archive(self.ptr) }
    }

    pub fn get_shadows_count(&self) -> usize {
        unsafe { sys::IMdl_module_info_get_shadows_count(self.ptr) }
    }

    pub fn get_shadow(&self, index: usize) -> Option<MdlModuleInfo> {
        let ptr = unsafe { sys::IMdl_module_info_get_shadow(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(MdlModuleInfo { ptr })
        }
    }
}

impl MdlInfo for MdlModuleInfo {
    fn as_mdl_info(&self) -> sys::IMdlInfo {
        self.ptr as sys::IMdlInfo
    }
}

impl Interface for MdlModuleInfo {
    fn from_interface_ptr(ptr: sys::IInterface) -> MdlModuleInfo {
        MdlModuleInfo {
            ptr: ptr as sys::IMdlModuleInfo,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMdl_module_info_type_get_iid() }
    }
}

pub struct MdlXliffInfo {
    pub(crate) ptr: sys::IMdlXliffInfo,
}

impl MdlXliffInfo {
    pub fn get_search_path_index(&self) -> usize {
        unsafe { sys::IMdl_xliff_info_get_search_path_index(self.ptr) }
    }

    pub fn get_search_path(&self) -> Option<String> {
        let ptr = unsafe { sys::IMdl_xliff_info_get_search_path(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }
    pub fn get_resolved_path(&self) -> Option<String> {
        let ptr = unsafe { sys::IMdl_xliff_info_get_resolved_path(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }
    pub fn in_archive(&self) -> bool {
        unsafe { sys::IMdl_xliff_info_in_archive(self.ptr) }
    }
}

impl Interface for MdlXliffInfo {
    fn from_interface_ptr(ptr: sys::IInterface) -> MdlXliffInfo {
        MdlXliffInfo {
            ptr: ptr as sys::IMdlXliffInfo,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMdl_xliff_info_type_get_iid() }
    }
}

impl MdlInfo for MdlXliffInfo {
    fn as_mdl_info(&self) -> sys::IMdlInfo {
        self.ptr as sys::IMdlInfo
    }
}

pub struct MdlDiscoveryApi {
    ptr: sys::discovery::IMdlDiscoveryApi,
}

impl MdlDiscoveryApi {
    pub fn discover(&self, filter: MdlInfoKind) -> Option<MdlDiscoveryResult> {
        let ptr = unsafe { sys::IMdl_discovery_api_discover(self.ptr, filter) };
        if ptr.is_null() {
            None
        } else {
            Some(MdlDiscoveryResult { ptr })
        }
    }
}

impl Interface for MdlDiscoveryApi {
    fn from_interface_ptr(ptr: sys::IInterface) -> MdlDiscoveryApi {
        MdlDiscoveryApi {
            ptr: ptr as sys::discovery::IMdlDiscoveryApi,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMdl_discovery_api_type_get_iid() }
    }
}

impl ApiComponent for MdlDiscoveryApi {}

pub struct MdlDiscoveryResult {
    ptr: sys::IMdlDiscoveryResult,
}

impl MdlDiscoveryResult {
    pub fn get_graph(&self) -> Option<MdlPackageInfo> {
        let ptr = unsafe { sys::IMdl_discovery_result_get_graph(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(MdlPackageInfo { ptr })
        }
    }

    pub fn get_search_paths_count(&self) -> usize {
        unsafe { sys::IMdl_discovery_result_get_search_paths_count(self.ptr) }
    }

    pub fn get_search_path(&self, index: usize) -> Option<String> {
        let ptr = unsafe {
            sys::IMdl_discovery_result_get_search_path(self.ptr, index)
        };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }
}

pub struct MdlPackageInfo {
    ptr: sys::IMdlPackageInfo,
}

impl MdlPackageInfo {
    pub fn get_child_count(&self) -> usize {
        unsafe { sys::IMdl_package_info_get_child_count(self.ptr) }
    }

    pub fn get_child(&self, index: usize) -> Option<MdlInfoBase> {
        let ptr = unsafe { sys::IMdl_package_info_get_child(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(MdlInfoBase { ptr })
        }
    }

    pub fn get_search_path_index_count(&self) -> usize {
        unsafe { sys::IMdl_package_info_get_search_path_index_count(self.ptr) }
    }

    pub fn get_search_path_index(&self, index: usize) -> usize {
        unsafe { sys::IMdl_package_info_get_search_path_index(self.ptr, index) }
    }

    pub fn get_search_path(&self, index: usize) -> Option<String> {
        let ptr =
            unsafe { sys::IMdl_package_info_get_search_path(self.ptr, index) };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

    pub fn get_resolved_path(&self, index: usize) -> Option<String> {
        let ptr = unsafe {
            sys::IMdl_package_info_get_resolved_path(self.ptr, index)
        };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

    pub fn in_archive(&self, index: usize) -> bool {
        unsafe { sys::IMdl_package_info_in_archive(self.ptr, index) }
    }
}

impl MdlInfo for MdlPackageInfo {
    fn as_mdl_info(&self) -> sys::IMdlInfo {
        self.ptr as sys::IMdlInfo
    }
}

impl Interface for MdlPackageInfo {
    fn from_interface_ptr(ptr: sys::IInterface) -> MdlPackageInfo {
        MdlPackageInfo {
            ptr: ptr as sys::IMdlPackageInfo,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMdl_package_info_type_get_iid() }
    }
}

pub struct MdlResourceInfo {
    ptr: sys::IMdlResourceInfo,
}

impl MdlResourceInfo {
    pub fn get_search_path_index(&self) -> usize {
        unsafe { sys::IMdl_resource_info_get_search_path_index(self.ptr) }
    }

    pub fn get_search_path(&self) -> Option<String> {
        let ptr = unsafe { sys::IMdl_resource_info_get_search_path(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }
    pub fn get_resolved_path(&self) -> Option<String> {
        let ptr =
            unsafe { sys::IMdl_resource_info_get_resolved_path(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

    pub fn get_extension(&self) -> Option<String> {
        let ptr = unsafe { sys::IMdl_resource_info_get_extension(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

    pub fn in_archive(&self) -> bool {
        unsafe { sys::IMdl_resource_info_in_archive(self.ptr) }
    }

    pub fn get_shadows_count(&self) -> usize {
        unsafe { sys::IMdl_resource_info_get_shadows_count(self.ptr) }
    }

    pub fn get_shadow(&self, index: usize) -> Option<MdlResourceInfo> {
        let ptr =
            unsafe { sys::IMdl_resource_info_get_shadow(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(MdlResourceInfo { ptr })
        }
    }
}

impl MdlInfo for MdlResourceInfo {
    fn as_mdl_info(&self) -> sys::IMdlInfo {
        self.ptr as sys::IMdlInfo
    }
}

impl Interface for MdlResourceInfo {
    fn from_interface_ptr(ptr: sys::IInterface) -> MdlResourceInfo {
        MdlResourceInfo {
            ptr: ptr as sys::IMdlResourceInfo,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMdl_resource_info_type_get_iid() }
    }
}


pub struct MdlTextureInfo {
    ptr: sys::IMdlTextureInfo,
}

impl MdlTextureInfo {
    pub fn get_search_path_index(&self) -> usize {
        unsafe { sys::IMdl_texture_info_get_search_path_index(self.ptr) }
    }

    pub fn get_search_path(&self) -> Option<String> {
        let ptr = unsafe { sys::IMdl_texture_info_get_search_path(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }
    pub fn get_resolved_path(&self) -> Option<String> {
        let ptr = unsafe { sys::IMdl_texture_info_get_resolved_path(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

    pub fn get_extension(&self) -> Option<String> {
        let ptr = unsafe { sys::IMdl_texture_info_get_extension(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

    pub fn in_archive(&self) -> bool {
        unsafe { sys::IMdl_texture_info_in_archive(self.ptr) }
    }

    pub fn get_shadows_count(&self) -> usize {
        unsafe { sys::IMdl_texture_info_get_shadows_count(self.ptr) }
    }

    pub fn get_shadow(&self, index: usize) -> Option<MdlResourceInfo> {
        let ptr = unsafe { sys::IMdl_texture_info_get_shadow(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(MdlResourceInfo { ptr })
        }
    }
}

impl MdlInfo for MdlTextureInfo {
    fn as_mdl_info(&self) -> sys::IMdlInfo {
        self.ptr as sys::IMdlInfo
    }
}

impl Interface for MdlTextureInfo {
    fn from_interface_ptr(ptr: sys::IInterface) -> MdlTextureInfo {
        MdlTextureInfo {
            ptr: ptr as sys::IMdlTextureInfo,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMdl_texture_info_type_get_iid() }
    }
}


pub struct MdlLightProfileInfo {
    ptr: sys::IMdlLightProfileInfo,
}

impl MdlLightProfileInfo {
    pub fn get_search_path_index(&self) -> usize {
        unsafe { sys::IMdl_lightprofile_info_get_search_path_index(self.ptr) }
    }

    pub fn get_search_path(&self) -> Option<String> {
        let ptr =
            unsafe { sys::IMdl_lightprofile_info_get_search_path(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }
    pub fn get_resolved_path(&self) -> Option<String> {
        let ptr =
            unsafe { sys::IMdl_lightprofile_info_get_resolved_path(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

    pub fn get_extension(&self) -> Option<String> {
        let ptr =
            unsafe { sys::IMdl_lightprofile_info_get_extension(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

    pub fn in_archive(&self) -> bool {
        unsafe { sys::IMdl_lightprofile_info_in_archive(self.ptr) }
    }

    pub fn get_shadows_count(&self) -> usize {
        unsafe { sys::IMdl_lightprofile_info_get_shadows_count(self.ptr) }
    }

    pub fn get_shadow(&self, index: usize) -> Option<MdlResourceInfo> {
        let ptr =
            unsafe { sys::IMdl_lightprofile_info_get_shadow(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(MdlResourceInfo { ptr })
        }
    }
}

impl MdlInfo for MdlLightProfileInfo {
    fn as_mdl_info(&self) -> sys::IMdlInfo {
        self.ptr as sys::IMdlInfo
    }
}

impl Interface for MdlLightProfileInfo {
    fn from_interface_ptr(ptr: sys::IInterface) -> MdlLightProfileInfo {
        MdlLightProfileInfo {
            ptr: ptr as sys::IMdlLightProfileInfo,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMdl_lightprofile_info_type_get_iid() }
    }
}


pub struct MdlMeasuredBsdfInfo {
    ptr: sys::IMdlMeasuredBsdfInfo,
}

impl MdlMeasuredBsdfInfo {
    pub fn get_search_path_index(&self) -> usize {
        unsafe { sys::IMdl_measured_bsdf_info_get_search_path_index(self.ptr) }
    }

    pub fn get_search_path(&self) -> Option<String> {
        let ptr =
            unsafe { sys::IMdl_measured_bsdf_info_get_search_path(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }
    pub fn get_resolved_path(&self) -> Option<String> {
        let ptr =
            unsafe { sys::IMdl_measured_bsdf_info_get_resolved_path(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

    pub fn get_extension(&self) -> Option<String> {
        let ptr =
            unsafe { sys::IMdl_measured_bsdf_info_get_extension(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string()
            })
        }
    }

    pub fn in_archive(&self) -> bool {
        unsafe { sys::IMdl_measured_bsdf_info_in_archive(self.ptr) }
    }

    pub fn get_shadows_count(&self) -> usize {
        unsafe { sys::IMdl_measured_bsdf_info_get_shadows_count(self.ptr) }
    }

    pub fn get_shadow(&self, index: usize) -> Option<MdlResourceInfo> {
        let ptr =
            unsafe { sys::IMdl_measured_bsdf_info_get_shadow(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(MdlResourceInfo { ptr })
        }
    }
}

impl MdlInfo for MdlMeasuredBsdfInfo {
    fn as_mdl_info(&self) -> sys::IMdlInfo {
        self.ptr as sys::IMdlInfo
    }
}

impl Interface for MdlMeasuredBsdfInfo {
    fn from_interface_ptr(ptr: sys::IInterface) -> MdlMeasuredBsdfInfo {
        MdlMeasuredBsdfInfo {
            ptr: ptr as sys::IMdlMeasuredBsdfInfo,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IMdl_measured_bsdf_info_type_get_iid() }
    }
}


pub enum Error {}
