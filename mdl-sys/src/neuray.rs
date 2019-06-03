use crate::components::*;
use crate::results::*;
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
    pub fn ineuray_get_api_component_database(n: INeuray) -> IDatabase;
    pub fn ineuray_get_api_component_debug_configuration(n: INeuray) -> IDebugConfiguration;
    pub fn ineuray_get_api_component_factory(n: INeuray) -> IFactory;
    pub fn ineuray_get_api_component_image_api(n: INeuray) -> IImageApi;
    pub fn ineuray_get_api_component_mdl_archive_api(n: INeuray) -> IMdlArchiveApi;
    pub fn ineuray_get_api_component_mdl_compiler(n: INeuray) -> IMdlCompiler;
    pub fn ineuray_get_api_component_discovery_api(n: INeuray) -> IMdlDiscoveryApi;
    pub fn ineuray_get_api_component_distiller_api(n: INeuray) -> IMdlDistillerApi;
    pub fn ineuray_get_api_component_evaluator_api(n: INeuray) -> IMdlEvaluatorApi;
    pub fn ineuray_get_api_component_mdl_factory(n: INeuray) -> IMdlFactory;
    pub fn ineuray_get_api_component_mdle_api(n: INeuray) -> IMdleApi;
    pub fn ineuray_get_api_component_version(n: INeuray) -> IVersion;
}
