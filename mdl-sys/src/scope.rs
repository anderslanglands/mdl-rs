use crate::transaction::ITransaction;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IScope_api {
    _unused: [u8; 0],
}
pub type IScope = *mut IScope_api;

extern "C" {
    pub fn IScope_release(s: IScope);
    pub fn IScope_retain(s: IScope);
    pub fn IScope_create_transaction(s: IScope) -> ITransaction;
}
