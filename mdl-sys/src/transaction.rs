use crate::{
    function_definition::IFunctionDefinition, interface::IInterface,
    material_definition::IMaterialDefinition, module::IModule,
};
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ITransaction_api {
    _unused: [u8; 0],
}
pub type ITransaction = *mut ITransaction_api;

extern "C" {
    pub fn ITransaction_release(s: ITransaction);
    pub fn ITransaction_retain(s: ITransaction);
    pub fn ITransaction_access_function_definition(
        s: ITransaction,
        name: *const c_char,
    ) -> IFunctionDefinition;
    pub fn ITransaction_access_material_definition(
        s: ITransaction,
        name: *const c_char,
    ) -> IMaterialDefinition;
    pub fn ITransaction_access(s: ITransaction, name: *const c_char) -> IInterface;
    pub fn ITransaction_access_module(s: ITransaction, name: *const c_char) -> IModule;
}
