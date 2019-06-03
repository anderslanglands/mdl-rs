#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IDatabase_api {
    _unused: [u8; 0],
}
pub type IDatabase = *mut IDatabase_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IDebugConfiguration_api {
    _unused: [u8; 0],
}
pub type IDebugConfiguration = *mut IDebugConfiguration_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IFactory_api {
    _unused: [u8; 0],
}
pub type IFactory = *mut IFactory_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IImageApi_api {
    _unused: [u8; 0],
}
pub type IImageApi = *mut IImageApi_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlArchiveApi_api {
    _unused: [u8; 0],
}
pub type IMdlArchiveApi = *mut IMdlArchiveApi_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlCompiler_api {
    _unused: [u8; 0],
}
pub type IMdlCompiler = *mut IMdlCompiler_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlDiscoveryApi_api {
    _unused: [u8; 0],
}
pub type IMdlDiscoveryApi = *mut IMdlDiscoveryApi_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlDistillerApi_api {
    _unused: [u8; 0],
}
pub type IMdlDistillerApi = *mut IMdlDistillerApi_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlEvaluatorApi_api {
    _unused: [u8; 0],
}
pub type IMdlEvaluatorApi = *mut IMdlEvaluatorApi_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdlFactory_api {
    _unused: [u8; 0],
}
pub type IMdlFactory = *mut IMdlFactory_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMdleApi_api {
    _unused: [u8; 0],
}
pub type IMdleApi = *mut IMdleApi_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IVersion_api {
    _unused: [u8; 0],
}
pub type IVersion = *mut IVersion_api;

