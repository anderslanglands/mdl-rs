use std::os::raw::c_char;

use crate::base::Uuid;

#[repr(u32)]
#[derive(Debug)]
pub enum MdlInfoKind {
    Package = 0,
    Module = 1,
    Xliff = 2,
    Texture = 4,
    LightProfile = 8,
    MeasuredBsdf = 16,
    Directory = 32,
    All = 0xffffffffu32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlPackageInfo_api {
    _unused: [u8; 0],
}
pub type IMdlPackageInfo = *mut IMdlPackageInfo_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlInfo_api {
    _unused: [u8; 0],
}
pub type IMdlInfo = *mut IMdlInfo_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlModuleInfo_api {
    _unused: [u8; 0],
}
pub type IMdlModuleInfo = *mut IMdlModuleInfo_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlXliffInfo_api {
    _unused: [u8; 0],
}
pub type IMdlXliffInfo = *mut IMdlXliffInfo_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlResourceInfo_api {
    _unused: [u8; 0],
}
pub type IMdlResourceInfo = *mut IMdlResourceInfo_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlTextureInfo_api {
    _unused: [u8; 0],
}
pub type IMdlTextureInfo = *mut IMdlTextureInfo_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlLightProfileInfo_api {
    _unused: [u8; 0],
}
pub type IMdlLightProfileInfo = *mut IMdlLightProfileInfo_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlMeasuredBsdfInfo_api {
    _unused: [u8; 0],
}
pub type IMdlMeasuredBsdfInfo = *mut IMdlMeasuredBsdfInfo_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlDiscoveryApi_api {
    _unused: [u8; 0],
}
pub type IMdlDiscoveryApi = *mut IMdlDiscoveryApi_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlDiscoveryResult_api {
    _unused: [u8; 0],
}
pub type IMdlDiscoveryResult = *mut IMdlDiscoveryResult_api;

extern "C" {
    pub fn IMdl_info_get_kind(i: IMdlInfo) -> MdlInfoKind;
    pub fn IMdl_info_get_qualified_name(i: IMdlInfo) -> *const c_char;
    pub fn IMdl_info_get_simple_name(i: IMdlInfo) -> *const c_char;
    pub fn IMdl_info_type_get_iid() -> Uuid;

    pub fn IMdl_module_info_get_search_path_index(
        info: IMdlModuleInfo,
    ) -> usize;
    pub fn IMdl_module_info_get_search_path(
        info: IMdlModuleInfo,
    ) -> *const c_char;
    pub fn IMdl_module_info_get_resolved_path(
        info: IMdlModuleInfo,
    ) -> *const c_char;
    pub fn IMdl_module_info_get_shadows_count(info: IMdlModuleInfo) -> usize;
    pub fn IMdl_module_info_get_shadow(
        info: IMdlModuleInfo,
        index: usize,
    ) -> IMdlModuleInfo;
    pub fn IMdl_module_info_in_archive(info: IMdlModuleInfo) -> bool;
    pub fn IMdl_module_info_type_get_iid() -> Uuid;

    /// Returns an absolute path to the xliff file in the local file system.
    pub fn IMdl_xliff_info_get_resolved_path(
        info: IMdlXliffInfo,
    ) -> *const c_char;
    /// Returns the index of the search path where the xliff file has been found.
    pub fn IMdl_xliff_info_get_search_path_index(info: IMdlXliffInfo) -> usize;
    /// Returns the search path in the local file system that contains this xliff file.
    pub fn IMdl_xliff_info_get_search_path(
        info: IMdlXliffInfo,
    ) -> *const c_char;
    /// Returns true if the xliff file has been discovered inside of an archive, false if not.
    pub fn IMdl_xliff_info_in_archive(info: IMdlXliffInfo) -> bool;
    pub fn IMdl_xliff_info_type_get_iid() -> Uuid;

    pub fn IMdl_discovery_api_type_get_iid() -> Uuid;
    pub fn IMdl_discovery_api_discover(
        d: IMdlDiscoveryApi,
        kind: MdlInfoKind,
    ) -> IMdlDiscoveryResult;

    pub fn IMdl_discovery_result_get_graph(
        r: IMdlDiscoveryResult,
    ) -> IMdlPackageInfo;

    pub fn IMdl_discovery_result_get_search_paths_count(
        r: IMdlDiscoveryResult,
    ) -> usize;

    pub fn IMdl_discovery_result_get_search_path(
        r: IMdlDiscoveryResult,
        index: usize,
    ) -> *const c_char;

    pub fn IMdl_package_info_get_child_count(i: IMdlPackageInfo) -> usize;
    pub fn IMdl_package_info_get_child(
        i: IMdlPackageInfo,
        index: usize,
    ) -> IMdlInfo;
    pub fn IMdl_package_info_get_search_path_index_count(
        i: IMdlPackageInfo,
    ) -> usize;
    pub fn IMdl_package_info_get_search_path_index(
        i: IMdlPackageInfo,
        index: usize,
    ) -> usize;
    pub fn IMdl_package_info_get_search_path(
        i: IMdlPackageInfo,
        index: usize,
    ) -> *const c_char;
    pub fn IMdl_package_info_get_resolved_path(
        i: IMdlPackageInfo,
        index: usize,
    ) -> *const c_char;
    pub fn IMdl_package_info_in_archive(
        i: IMdlPackageInfo,
        index: usize,
    ) -> bool;
    pub fn IMdl_package_info_type_get_iid() -> Uuid;

    pub fn IMdl_resource_info_get_search_path_index(
        info: IMdlResourceInfo,
    ) -> usize;
    pub fn IMdl_resource_info_get_search_path(
        info: IMdlResourceInfo,
    ) -> *const c_char;
    pub fn IMdl_resource_info_get_resolved_path(
        info: IMdlResourceInfo,
    ) -> *const c_char;
    pub fn IMdl_resource_info_get_shadows_count(
        info: IMdlResourceInfo,
    ) -> usize;
    pub fn IMdl_resource_info_get_shadow(
        info: IMdlResourceInfo,
        index: usize,
    ) -> IMdlResourceInfo;
    pub fn IMdl_resource_info_in_archive(info: IMdlResourceInfo) -> bool;
    pub fn IMdl_resource_info_type_get_iid() -> Uuid;
    pub fn IMdl_resource_info_get_extension(
        info: IMdlResourceInfo,
    ) -> *const c_char;

    pub fn IMdl_texture_info_get_search_path_index(
        info: IMdlTextureInfo,
    ) -> usize;
    pub fn IMdl_texture_info_get_search_path(
        info: IMdlTextureInfo,
    ) -> *const c_char;
    pub fn IMdl_texture_info_get_resolved_path(
        info: IMdlTextureInfo,
    ) -> *const c_char;
    pub fn IMdl_texture_info_get_shadows_count(info: IMdlTextureInfo) -> usize;
    pub fn IMdl_texture_info_get_shadow(
        info: IMdlTextureInfo,
        index: usize,
    ) -> IMdlResourceInfo;
    pub fn IMdl_texture_info_in_archive(info: IMdlTextureInfo) -> bool;
    pub fn IMdl_texture_info_type_get_iid() -> Uuid;
    pub fn IMdl_texture_info_get_extension(
        info: IMdlTextureInfo,
    ) -> *const c_char;


    pub fn IMdl_lightprofile_info_get_search_path_index(
        info: IMdlLightProfileInfo,
    ) -> usize;
    pub fn IMdl_lightprofile_info_get_search_path(
        info: IMdlLightProfileInfo,
    ) -> *const c_char;
    pub fn IMdl_lightprofile_info_get_resolved_path(
        info: IMdlLightProfileInfo,
    ) -> *const c_char;
    pub fn IMdl_lightprofile_info_get_shadows_count(
        info: IMdlLightProfileInfo,
    ) -> usize;
    pub fn IMdl_lightprofile_info_get_shadow(
        info: IMdlLightProfileInfo,
        index: usize,
    ) -> IMdlResourceInfo;
    pub fn IMdl_lightprofile_info_in_archive(
        info: IMdlLightProfileInfo,
    ) -> bool;
    pub fn IMdl_lightprofile_info_type_get_iid() -> Uuid;
    pub fn IMdl_lightprofile_info_get_extension(
        info: IMdlLightProfileInfo,
    ) -> *const c_char;


    pub fn IMdl_measured_bsdf_info_get_search_path_index(
        info: IMdlMeasuredBsdfInfo,
    ) -> usize;
    pub fn IMdl_measured_bsdf_info_get_search_path(
        info: IMdlMeasuredBsdfInfo,
    ) -> *const c_char;
    pub fn IMdl_measured_bsdf_info_get_resolved_path(
        info: IMdlMeasuredBsdfInfo,
    ) -> *const c_char;
    pub fn IMdl_measured_bsdf_info_get_shadows_count(
        info: IMdlMeasuredBsdfInfo,
    ) -> usize;
    pub fn IMdl_measured_bsdf_info_get_shadow(
        info: IMdlMeasuredBsdfInfo,
        index: usize,
    ) -> IMdlResourceInfo;
    pub fn IMdl_measured_bsdf_info_in_archive(
        info: IMdlMeasuredBsdfInfo,
    ) -> bool;
    pub fn IMdl_measured_bsdf_info_type_get_iid() -> Uuid;
    pub fn IMdl_measured_bsdf_info_get_extension(
        info: IMdlMeasuredBsdfInfo,
    ) -> *const c_char;


}
