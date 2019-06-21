use mdl_sys as sys;

use crate::base::Interface;

use err_derive::Error;

pub struct Neuray {
    n: sys::INeuray,
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub trait ApiComponent: Interface {}


impl Neuray {
    pub fn new() -> Result<Neuray> {
        let n = unsafe { sys::load_ineuray() };
        if n.is_null() {
            return Err(Error::LoadNeurayFailed);
        } else {
            Ok(Neuray { n })
        }
    }

    pub fn start(&mut self) -> Result<()> {
        let result = unsafe { sys::ineuray_start(self.n) };
        if result != sys::INeurayStartResult::Success {
            return Err(Error::from(result));
        } else {
            Ok(())
        }
    }

    pub fn get_api_component<C: ApiComponent>(&self) -> Result<C> {
        let ptr =
            unsafe { sys::ineuray_get_api_component(self.n, C::type_iid()) };
        if ptr.is_null() {
            return Err(Error::GetApiComponentFailed);
        }
        Ok(C::from_interface(ptr))
    }
}

impl Drop for Neuray {
    fn drop(&mut self) {
        if unsafe { sys::ineuray_shutdown(self.n) }
            != sys::INeurayShutdownResult::Success
        {
            panic!("ineuray shutdown failed");
        }
    }
}

unsafe impl Send for Neuray {}

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "Could not load ineuray interface from DSO")]
    LoadNeurayFailed,
    #[error(display = "Unspecified internal failure")]
    UnspecifiedFailure,
    #[error(display = "Challenge-response authentication failure")]
    ChallengeResponseAuthenticationFailure,
    #[error(display = "SPM authentication failure")]
    SpmAuthenticationFailure,
    #[error(display = "Provided license expired")]
    ProvidedLicenseExpired,
    #[error(
        display = "No professional GPU found as required by the license terms"
    )]
    NoProfessionalGpuFound,
    #[error(display = "FlexLM authentication failure")]
    FlexLmAuthenticationFailure,
    #[error(display = "No NVidia VCA found as required by the license terms")]
    NoNvidiaVcaFound,
    #[error(display = "Failed to get api component")]
    GetApiComponentFailed,
}

impl From<sys::INeurayStartResult> for Error {
    fn from(start_result: sys::INeurayStartResult) -> Error {
        match start_result {
            sys::INeurayStartResult::Success => unreachable!(),
            sys::INeurayStartResult::UnspecifiedFailure => {
                Error::UnspecifiedFailure
            }
            sys::INeurayStartResult::ChallengeResponseAuthenticationFailure => {
                Error::ChallengeResponseAuthenticationFailure
            }
            sys::INeurayStartResult::SpmAuthenticationFailure => {
                Error::SpmAuthenticationFailure
            }
            sys::INeurayStartResult::ProvidedLicenseExpired => {
                Error::ProvidedLicenseExpired
            }
            sys::INeurayStartResult::NoProfessionalGpuFound => {
                Error::NoProfessionalGpuFound
            }
            sys::INeurayStartResult::FlexLmAuthenticationFailure => {
                Error::FlexLmAuthenticationFailure
            }
            sys::INeurayStartResult::NoNvidiaVcaFound => {
                Error::NoNvidiaVcaFound
            }
        }
    }
}
