use mdl_sys as sys;

use crate::Version;

use snafu::{ensure, Snafu};

pub struct Neuray {
    n: sys::INeuray,
}

type Result<T, E = Error> = std::result::Result<T, E>;

impl Neuray {
    pub fn new() -> Result<Neuray> {
        let n = unsafe { sys::load_ineuray() };
        ensure!(!n.is_null(), LoadNeurayFailed);
        Ok(Neuray { n })
    }

    pub fn start(&mut self) -> Result<()> {
        let result = unsafe { sys::ineuray_start(self.n) };
        if result != sys::INeurayStartResult::Success {
            return Err(Error::from(result));
        } else {
            Ok(())
        }
    }

    // pub fn get_api_component_database(&self) -> IDatabase;
    // pub fn get_api_component_debug_configuration(&self) -> IDebugConfiguration;
    // pub fn get_api_component_factory(&self) -> IFactory;
    // pub fn get_api_component_image_api(&self) -> IImageApi;
    // pub fn get_api_component_mdl_archive_api(&self) -> IMdlArchiveApi;
    // pub fn get_api_component_mdl_compiler(&self) -> IMdlCompiler;
    // pub fn get_api_component_discovery_api(&self) -> IMdlDiscoveryApi;
    // pub fn get_api_component_distiller_api(&self) -> IMdlDistillerApi;
    // pub fn get_api_component_evaluator_api(&self) -> IMdlEvaluatorApi;
    // pub fn get_api_component_mdl_factory(&self) -> IMdlFactory;
    // pub fn get_api_component_mdle_api(&self) -> IMdleApi;
    pub fn get_api_component_version(&self) -> Result<Version> {
        let v = unsafe { sys::ineuray_get_api_component_version(self.n) };
        if v.is_null() {
            Err(Error::GetApiComponentFailed)
        } else {
            Ok({ Version { v } })
        }
    }
}

impl Drop for Neuray {
    fn drop(&mut self) {
        if unsafe { sys::ineuray_shutdown(self.n) } != sys::INeurayShutdownResult::Success {
            panic!("ineuray shutdown failed");
        }
    }
}

unsafe impl Send for Neuray {}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not load ineuray interface from DSO"))]
    LoadNeurayFailed,
    #[snafu(display("Unspecified internal failure"))]
    UnspecifiedFailure,
    #[snafu(display("Challenge-response authentication failure"))]
    ChallengeResponseAuthenticationFailure,
    #[snafu(display("SPM authentication failure"))]
    SpmAuthenticationFailure,
    #[snafu(display("Provided license expired"))]
    ProvidedLicenseExpired,
    #[snafu(display("No professional GPU found as required by the license terms"))]
    NoProfessionalGpuFound,
    #[snafu(display("FlexLM authentication failure"))]
    FlexLmAuthenticationFailure,
    #[snafu(display("No NVidia VCA found as required by the license terms"))]
    NoNvidiaVcaFound,
    #[snafu(display("Failed to get api component"))]
    GetApiComponentFailed,
}

impl From<sys::INeurayStartResult> for Error {
    fn from(start_result: sys::INeurayStartResult) -> Error {
        match start_result {
            sys::INeurayStartResult::Success => unreachable!(),
            sys::INeurayStartResult::UnspecifiedFailure => Error::UnspecifiedFailure,
            sys::INeurayStartResult::ChallengeResponseAuthenticationFailure => {
                Error::ChallengeResponseAuthenticationFailure
            }
            sys::INeurayStartResult::SpmAuthenticationFailure => Error::SpmAuthenticationFailure,
            sys::INeurayStartResult::ProvidedLicenseExpired => Error::ProvidedLicenseExpired,
            sys::INeurayStartResult::NoProfessionalGpuFound => Error::NoProfessionalGpuFound,
            sys::INeurayStartResult::FlexLmAuthenticationFailure => {
                Error::FlexLmAuthenticationFailure
            }
            sys::INeurayStartResult::NoNvidiaVcaFound => Error::NoNvidiaVcaFound,
        }
    }
}
