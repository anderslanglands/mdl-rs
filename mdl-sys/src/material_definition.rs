#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMaterialDefinition_api {
    _unused: [u8; 0],
}
pub type IMaterialDefinition = *mut IMaterialDefinition_api;

extern "C" {
    pub fn IMaterial_definition_release(s: IMaterialDefinition);
    pub fn IMaterial_definition_retain(s: IMaterialDefinition);
}


