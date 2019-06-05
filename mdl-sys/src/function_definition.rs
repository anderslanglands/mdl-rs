#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IFunctionDefinition_api {
    _unused: [u8; 0],
}
pub type IFunctionDefinition = *mut IFunctionDefinition_api;

extern "C" {
    pub fn IFunction_definition_release(s: IFunctionDefinition);
    pub fn IFunction_definition_retain(s: IFunctionDefinition);
}

