#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IScope_api {
    _unused: [u8; 0],
}
pub type IScope = *mut IScope_api;

extern "C" {
    pub fn IScope_release(s: IScope);
}

